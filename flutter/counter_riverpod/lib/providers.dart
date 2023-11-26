import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'providers.g.dart';

/// 此处 annotation 生成的是一个 NotifierProvider
@riverpod
class MyCounter extends _$MyCounter {
  @override
  int build() => 20;

  void increment() {
    state++;
  }
}

// 下面这个annotation 生成的是一个 Provider
@riverpod
int myCounterAlt(MyCounterAltRef ref) {
  return ref.watch(myCounterProvider);
}

class Counter extends StateNotifier<int> {
  Counter() : super(0);

  void increment() {
    state++;
  }
}

// 生成一个 StreamNotifierProvider
@riverpod
class Values extends _$Values {
  @override
  Stream<int> build() {
    return Stream.fromIterable([1, 2, 3]);
  }
}

// 生成一个 StreamNotifierProvider
@riverpod
class Example extends _$Example {
  @override
  Stream<String> build() async* {
    yield 'foo';
  }
  // Add methods to mutate the state
}

// final counterProvider = StateNotifierProvider((ref) => Counter());
final counterProvider = StateNotifierProvider<Counter, int>((ref) {
  return Counter();
});

class Todo {
  Todo(this.description, this.isCompleted);
  final bool isCompleted;
  final String description;
}

@riverpod
class Todos extends _$Todos {
  @override
  List<Todo> build() {
    return [];
  }

  void addTodo(Todo todo) {
    state = [...state, todo];
  }
}

@riverpod
List<Todo> completedTodos(CompletedTodosRef ref) {
  final todos = ref.watch(todosProvider);

  // 我们只返回完成的待办事项
  return todos.where((todo) => todo.isCompleted).toList();
}
