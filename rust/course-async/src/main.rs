use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
    time::Duration,
};

use course_async::TimerFuture;
use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

// impl Executor {
//     fn run(&self) {
//         while let Ok(task) = self.ready_queue.recv() {
//             let mut future_slot = task.future.lock().unwrap();
//             if let Some(mut future) = future_slot.take() {
//                 let waker = waker_ref(&task);
//                 let context = &mut Context::from_waker(&*waker);
//                 if future.as_mut().poll(context).is_pending() {
//                     *future_slot = Some(future);
//                 }
//             }
//         }
//     }
// }

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("Task Queue is full.");
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("Task Queue is full.");
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

// fn main() {
//     let (executor, spawner) = new_executor_and_spawner();

//     spawner.spawn(async {
//         println!("howdy!");
//         TimerFuture::new(Duration::new(2, 0)).await;
//         println!("done!");
//     });

//     // 会导致 Line 23 .recv() 可以退出，因为发送端已经关闭。进而 executor 得以退出
//     drop(spawner);

//     executor.run();
// }

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 如果发送端还在，且ready_queue里没有任务，主线程就在此阻塞
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                //
                // future是spawner.spawn的整个async块。第一次poll推动执行了块内await前的同步代码，即打印出"任务开始"
                // 同步代码执行完TimerFuture::new后返回一个类型为TimerFuture的future,
                // 之后的.await驱动执行TimerFuture的poll方法，顺便通过TimerFuture的poll方法，
                // 把下面future.as_mut().poll(context)传入的context(包含了waker)接续传入TimerFuture内部
                //
                // 第一次对TimerFuture做poll返回了Poll::Pending，也就是在.await处返回Poll::Pending
                // 那次轮执行外层task就在.await处保存好整个外层task的栈现场（task内.await后的代码不再执行），然后让出主线程的控制权.
                //
                // 主线程不可能被挂起,所以继续执行后续同步代码。
                // 因为此轮while循环外层future的第一次poll返回了Poll::Pending,所以继续执行if块内部代码
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    // 之所以再放回，是因为前面用了take()。但如果前面不用take()，这里也不用放回了，是否可行？
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    // 将外层task发送到 executor 和 spawner 所在的同步通道中
    spawner.spawn(async {
        println!("howdy!");
        //等待TimerFuture睡醒后执行外层task的wake_by_ref将此外层task再次发送到此同步通道中
        TimerFuture::new(Duration::from_secs(2)).await;
        println!("done!");
    });
    drop(spawner); // 因为spawn里复制了一份发送端给外层task，所以spawner被删除后，上面建立的同步通道仍在，下面一句仍能正常执行
    executor.run();
}
