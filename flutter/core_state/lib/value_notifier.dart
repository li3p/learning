import 'dart:math';

import 'package:flutter/material.dart';

class NotifierWidget extends StatefulWidget {
  final ValueNotifier<int> data;

  const NotifierWidget({super.key, required this.data});

  @override
  State<NotifierWidget> createState() => _NotifierWidgetState();
}

class _NotifierWidgetState extends State<NotifierWidget> {
  int _counter = 0;

  @override
  void initState() {
    super.initState();
    widget.data.addListener(onNotify);
    _counter = widget.data.value;
  }

  void onNotify() {
    setState(() => _counter = widget.data.value);
  }

  @override
  Widget build(BuildContext context) {
    return Text(
      '$_counter',
      style: Theme.of(context).textTheme.headlineMedium,
    );
  }

  @override
  @override
  void dispose() {
    widget.data.removeListener(onNotify);
    super.dispose();
  }
}

class NotifyButton extends StatelessWidget {
  final ValueNotifier<int> data;
  const NotifyButton({super.key, required this.data});

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      onPressed: () => data.value = Random().nextInt(100),
      tooltip: 'Increment',
      child: const Icon(Icons.add),
    );
  }
}

class MyValueNotifierPage extends StatelessWidget {
  MyValueNotifierPage({super.key});

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
          // Column is also a layout widget. It takes a list of children and
          // arranges them vertically. By default, it sizes itself to fit its
          // children horizontally, and tries to be as tall as its parent.
          //
          // Column has various properties to control how it sizes itself and
          // how it positions its children. Here we use mainAxisAlignment to
          // center the children vertically; the main axis here is the vertical
          // axis because Columns are vertical (the cross axis would be
          // horizontal).
          //
          // TRY THIS: Invoke "debug painting" (choose the "Toggle Debug Paint"
          // action in the IDE, or press "p" in the console), to see the
          // wireframe for each widget.
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'Number notified by ValueNotifer<int>:',
            ),
            // Text(
            //   '_counter',
            //   style: Theme.of(context).textTheme.headlineMedium,
            // ),
            NotifierWidget(data: _counterVal),
          ],
        ),
      ),
      floatingActionButton: NotifyButton(
          data:
              _counterVal), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}
