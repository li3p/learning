import 'package:flutter_riverpod/flutter_riverpod.dart';

class Counter extends StateNotifier<int> {
  Counter() : super(0);

  void increment() {
    state++;
  }
}

// final counterProvider = StateNotifierProvider((ref) => Counter());
final counterProvider = StateNotifierProvider<Counter, int>((ref) {
  return Counter();
});
