import 'package:equatable/equatable.dart';

class CounterState extends Equatable {
  final int counter;

  const CounterState({this.counter = 0});

// 覆盖 props 属性。
// 这是一个列表，其中包含使整个类等于同一类型的另一个类所需的所有属性。
  @override
  List<Object> get props => [counter];
}
