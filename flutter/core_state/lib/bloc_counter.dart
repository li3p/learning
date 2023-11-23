import 'package:core_state/bloc_counter_event.dart';
import 'package:core_state/bloc_counter_state.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class CounterBloc extends Bloc<CounterEvent, CounterState> {
  CounterBloc() : super(initialState) {
    on<IncrementCounterEvent>(mapEventToState);
    on<AddToCounterEvent>(mapEventToState);
  }

  int countValue = 0;

  static get initialState => const CounterState();

  Future<void> mapEventToState(
      CounterEvent event, Emitter<CounterState> emit) async {
    if (event is IncrementCounterEvent) {
      emit(CounterState(counter: ++countValue));
    }
    if (event is AddToCounterEvent) {
      emit(CounterState(counter: countValue += event.number));
    }
  }
}
