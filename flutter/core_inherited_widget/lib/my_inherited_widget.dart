import 'package:flutter/material.dart';

class MyInheritedWidget extends InheritedWidget {
  final int counter;
  const MyInheritedWidget(
      {super.key, required this.counter, required super.child});

  @override
  bool updateShouldNotify(covariant InheritedWidget oldWidget) {
    return counter != (oldWidget as MyInheritedWidget).counter;
  }

  static MyInheritedWidget of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<MyInheritedWidget>()!;
  }
}
