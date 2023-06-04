import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'main.dart';

class CounterModel {
  int count;
  CounterModel({required this.count});
}

class CounterNotifier extends StateNotifier<CounterModel> {
  CounterNotifier() : super(CounterModel(count: 4));

  void increment() => state = CounterModel(count: ++state.count);
}

class RiverpodCustomCounterExample extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(counterStateProvider) as CounterModel;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Riverpod Custom Counter Example'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '${state.count}',
              style: Theme.of(context).textTheme.headlineMedium,
            )
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => ref.read(counterStateProvider.notifier).increment(),
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
