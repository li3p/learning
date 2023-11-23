import 'package:flutter/material.dart';

class MyInheritedWidget extends InheritedWidget {
  final MyCounterInheritedWidgetState data;
  const MyInheritedWidget(
      {super.key, required super.child, required this.data});

  @override
  bool updateShouldNotify(covariant InheritedWidget oldWidget) {
    // return counter != (oldWidget as MyInheritedWidget).counter;
    return child != oldWidget;
  }

  // static MyInheritedWidget of(BuildContext context) {
  //   return context.dependOnInheritedWidgetOfExactType<MyInheritedWidget>()!;
  // }
}

class MyCounterInheritedWidget extends StatefulWidget {
  final Widget child;
  const MyCounterInheritedWidget({super.key, required this.child});

  @override
  State<StatefulWidget> createState() => MyCounterInheritedWidgetState();

  static MyCounterInheritedWidgetState of(BuildContext context) {
    // final MyInheritedWidget myInheritedWidget =
    //     context.dependOnInheritedWidgetOfExactType<MyInheritedWidget>()!;

    // if (null == myInheritedWidget) {
    //   throw Exception('No MyInheritedWidget found in context');
    // }
    // final MyCounterInheritedWidgetState? result =
    //     context.dependOnInheritedWidgetOfExactType<MyInheritedWidget>()!.data;
    // assert(result != null, 'No MyInheritedWidget found in context');
    return context
            .dependOnInheritedWidgetOfExactType<MyInheritedWidget>()
            ?.data ??
        (throw FlutterError('No MyInheritedWidget found in context'));
  }

  // static MyCounterInheritedWidgetState of2(BuildContext context) {
  //   final MyCounterInheritedWidgetState? result =
  //       context.findAncestorWidgetOfExactType<MyInheritedWidget>()!.data;
  //   assert(result != null, 'No MyInheritedWidget found in context');
  //   return result!;
  // }

  static MyCounterInheritedWidgetState of2(BuildContext context) {
    return context.findAncestorWidgetOfExactType<MyInheritedWidget>()?.data ??
        (throw FlutterError('No MyInheritedWidget found in context'));
  }
}

class MyCounterInheritedWidgetState extends State<MyCounterInheritedWidget> {
  int _counterValue = 0;
  int get counterValue => _counterValue;
  void increamentCounter() {
    setState(() {
      _counterValue++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MyInheritedWidget(
      data: this,
      child: widget.child,
    );
  }
}
