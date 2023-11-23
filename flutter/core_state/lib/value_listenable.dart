import 'dart:math';

import 'package:flutter/material.dart';

class MyValueListenerPage extends StatelessWidget {
  MyValueListenerPage({super.key});

  final ValueNotifier<int> _counterVal = ValueNotifier<int>(0);

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      appBar: AppBar(
        // TRY THIS: Try changing the color here to a specific color (to
        // Colors.amber, perhaps?) and trigger a hot reload to see the AppBar
        // change color while the other colors stay the same.
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: const Text("Value Notifier"),
      ),
      body: Center(
        // Center is a layout widget. It takes a single child and positions it
        // in the middle of the parent.
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
