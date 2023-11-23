import 'dart:math';

import 'package:flutter/material.dart';

class MyCounterNotification extends Notification {
  final int counter;
  const MyCounterNotification({required this.counter});
}

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
                    _counter = notification.counter;
                  });
                  return true;
                },
                child: Column(
                  children: [
                    Text(
                      '$_counter',
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
