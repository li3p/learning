import 'dart:math';

import 'package:flutter/material.dart';

class MyValueListenerPage extends StatelessWidget {
  MyValueListenerPage({super.key});

  final ValueNotifier<int> _counterVal = ValueNotifier<int>(0);

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
              'Number notified by ValueNotifer<int>:',
            ),
            ValueListenableBuilder<int>(
              valueListenable: _counterVal,
              builder: (BuildContext context, int value, Widget? child) {
                return Text(
                  '$value',
                  style: Theme.of(context).textTheme.headlineMedium,
                );
              },
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _counterVal.value = Random().nextInt(100),
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
