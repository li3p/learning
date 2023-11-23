import 'dart:math';

import 'package:flutter/material.dart';

// 相当于数据类，承载数据状态
class MyCounterNotification extends Notification {
  final int counter;
  const MyCounterNotification({required this.counter});
}

// 封装一个按钮，点击后发送通知，被 NotificationListener 包裹做为子组件
class MyNotificationWidget extends StatelessWidget {
  const MyNotificationWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
        onPressed: () {
          MyCounterNotification(counter: Random().nextInt(100))
              .dispatch(context);
        },
        child: const Text("Next Number"));
  }
}

// 监听通知的页面, 通过 NotificationListener 包裹子组件，监听通知
// 另外有一个与 NotificationListener 同级的，通知是监听不到的。
class MyNotificationListenerPage extends StatefulWidget {
  const MyNotificationListenerPage({super.key});

  @override
  State<StatefulWidget> createState() => _MyNotificationListenerPageState();
}

class _MyNotificationListenerPageState
    extends State<MyNotificationListenerPage> {
  int _counter = 10;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: const Text("Value Notifier"),
      ),
      body: Center(
        child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              const Text(
                'Number notified by Notification<int>:',
              ),
              NotificationListener<MyCounterNotification>(
                onNotification: (notification) {
                  setState(() {
                    _counter = notification.counter; // 通过通知更新数据
                  });
                  return true;
                },
                child: Column(
                  children: [
                    Text(
                      '$_counter', // 因为要展示数据，所以需要MyNotificationListenerPage是StatefulWidget
                      style: Theme.of(context).textTheme.headlineMedium,
                    ),
                    const MyNotificationWidget(),
                  ],
                ),
              ),
            ]),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          MyCounterNotification(counter: Random().nextInt(100))
              .dispatch(context);
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
