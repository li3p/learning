import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Counter with ChangeNotifier {
  int _count = 0;
  int get count => _count;
  void increment() {
    _count++;
    notifyListeners();
  }
}

class ProviderCounterExamplePage extends StatelessWidget {
  const ProviderCounterExamplePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Provider Counter Example'),
      ),
      body: const Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              'You have pushed the button this may times:',
            ),
            // Text('${Provider.of<Counter>(context)._count}',
            // Text('${context.watch<Counter>()._count}',
            //     style: Theme.of(context).textTheme.headlineMedium),
            MyCounter(),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () =>
            // Provider.of<Counter>(context, listen: false).increment(),
            context.read<Counter>().increment(),
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}

class MyCounter extends StatelessWidget {
  const MyCounter({super.key});

  @override
  Widget build(BuildContext context) {
    return Text('${context.watch<Counter>()._count}',
        style: Theme.of(context).textTheme.headlineMedium);
  }
}
