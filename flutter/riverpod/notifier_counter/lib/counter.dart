import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'main.dart';

class CounterNotifier extends Notifier<int> {
  @override
  int build() {
    return 35;
  }

  void increment() {
    state++;
  }
}

final counterProvider =
    NotifierProvider<CounterNotifier, int>(CounterNotifier.new);

class CounterWidget extends ConsumerWidget {
  const CounterWidget({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final counter = ref.watch(counterProvider);
    return ElevatedButton(
      // onPressed: () => ref.read(counterProvider.notifier).state++,
      onPressed: () => ref.read(counterProvider.notifier).increment(),
      child: Text('Value: $counter'),
    );
  }
}
