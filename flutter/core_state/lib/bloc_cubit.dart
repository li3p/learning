import 'package:core_state/bloc_counter.dart';
import 'package:core_state/bloc_counter_state.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class MyBlocCubitCounterPage extends StatelessWidget {
  const MyBlocCubitCounterPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider<CounterCubit>(
      create: (_) => CounterCubit(),
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
          child: BlocBuilder<CounterCubit, CounterState>(
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
              onPressed: () => context.read<CounterCubit>().increment(),
              tooltip: 'Increment',
              child: const Icon(Icons.add),
            ),
            FloatingActionButton(
              onPressed: () => context.read<CounterCubit>().decrement(),
              tooltip: 'Decrement',
              child: const Icon(Icons.remove),
            ),
          ],
        ));
  }
}

class CounterCubit extends Cubit<CounterState> {
  CounterCubit() : super(const CounterState());

  void increment() => emit(CounterState(counter: state.counter + 1));

  void decrement() => emit(CounterState(counter: state.counter - 1));
}
