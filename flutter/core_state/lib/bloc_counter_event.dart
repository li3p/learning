class CounterEvent {}

// UI 中需要增加计数的事件
class IncrementCounterEvent extends CounterEvent {}

// UI 中将计数增加特定数量的事件
class AddToCounterEvent extends CounterEvent {
  final int number;
  AddToCounterEvent(this.number);
}
