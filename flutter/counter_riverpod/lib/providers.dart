import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'providers.g.dart';

@riverpod
class MyCounter extends _$MyCounter {
  @override
  int build() => 20;

  void increment() {
    state++;
  }
}

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
