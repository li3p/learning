import 'dart:math';

import 'package:core_state/bloc_counter.dart';
import 'package:core_state/bloc_counter_event.dart';
import 'package:core_state/bloc_counter_state.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

/// flutter pub add flutter_bloc
/// 1. Creating State classes
/// 2. Creating Event classes
/// 3. Create a Bloc class
/// 4. Consuming the bloc class in our UI
/// 5. Wrap the MaterialApp with BlocProvider
///
///

class MyBlocCounterPage extends StatelessWidget {
  const MyBlocCounterPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: const Text("Value Notifier"),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'Number notified by ValueNotifer<int>:',
            ),
            BlocBuilder(
              bloc: BlocProvider.of<CounterBloc>(context),
              builder: (context, state) {
                return Text(
                  '${(state as CounterState).counter}',
                  style: Theme.of(context).textTheme.headlineMedium,
                );
              },
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => BlocProvider.of<CounterBloc>(context)
            .add(AddToCounterEvent(Random().nextInt(100))),
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
