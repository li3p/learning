import 'dart:async';
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(),
    );
  }
}

class MyHomePage extends HookWidget {
  const MyHomePage({super.key});

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  @override
  Widget build(BuildContext context) {
    final numberNotifier = useState(0);
    var duration = const Duration(seconds: 1);
    final animationController = useAnimationController(duration: duration)
      ..repeat(reverse: false);

    useEffect(() {
      print('useEffect');
      final timer = Timer.periodic(duration, (timer) {
        numberNotifier.value = timer.tick;
      });
      return () {
        print('useEffect cleanup');
        timer.cancel();
        animationController.stop();
      };
    }, []);

    return Scaffold(
      body: Center(
        child: AnimatedBuilder(
          animation: animationController,
          builder: (BuildContext context, Widget? child) {
            return Transform.rotate(
              angle: animationController.value * 2 * pi,
              child: Text(
                numberNotifier.value.toString(),
                style: Theme.of(context).textTheme.headlineLarge,
              ),
            );
          },
        ),
      ),
    );
  }
}
