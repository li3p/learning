import 'package:core_state/bloc_counter.dart';
import 'package:core_state/bloc_counter_state.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class MyBlocCubitCounterPage extends StatelessWidget {
  const MyBlocCubitCounterPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (_) => CounterCubitBloc(),
      child: const CounterCubitView(),
    );
  }
}

class CounterCubitView extends StatelessWidget {
  const CounterCubitView({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          backgroundColor: Theme.of(context).colorScheme.inversePrimary,
          title: const Text("Bloc Cubit Counter"),
        ),
        body: Center(
          child: BlocBuilder<CounterCubitBloc, CounterState>(
            builder: (context, state) {
              return Text(
                '${state.counter}',
                style: Theme.of(context).textTheme.displayLarge,
              );
            },
          ),
        ),
        floatingActionButton: Row(
          crossAxisAlignment: CrossAxisAlignment.end,
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            FloatingActionButton(
              onPressed: () => context
                  .read<CounterCubitBloc>()
                  .add(CounterIncrementPressed()),
              tooltip: 'Increment',
              child: const Icon(Icons.add),
            ),
            FloatingActionButton(
              onPressed: () => context
                  .read<CounterCubitBloc>()
                  .add(CounterDecrementPressed()),
              tooltip: 'Decrement',
              child: const Icon(Icons.remove),
            ),
          ],
        ));
  }
}

/// Event being processed by [CounterBloc].
abstract class CounterEvent {}

/// Notifies bloc to increment state.
class CounterIncrementPressed extends CounterEvent {}

/// Notifies bloc to decrement state.
class CounterDecrementPressed extends CounterEvent {}

/// {@template counter_bloc}
/// A simple [Bloc] that manages an `int` as its state.
/// {@endtemplate}
class CounterCubitBloc extends Bloc<CounterEvent, CounterState> {
  /// {@macro counter_bloc}
  CounterCubitBloc() : super(const CounterState()) {
    on<CounterIncrementPressed>(
        (event, emit) => emit(CounterState(counter: state.counter + 1)));
    on<CounterDecrementPressed>(
        (event, emit) => emit(CounterState(counter: state.counter - 1)));
  }
}
