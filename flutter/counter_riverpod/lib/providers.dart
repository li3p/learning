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
