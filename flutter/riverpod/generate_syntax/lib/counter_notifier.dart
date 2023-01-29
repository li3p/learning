import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'counter_notifier.g.dart';

@riverpod
class Counter extends _$Counter {
  @override
  int build() {
    return 0;
  }

  void increment() {
    state++;
  }
}

// final counterProvider = NotifierProvider<Counter, int>(Counter.new);
