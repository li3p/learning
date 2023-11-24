import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class ProviderCounter extends ChangeNotifier {
  int _count = 0;

  int get count => _count;

  void increment() {
    _count++;
    notifyListeners();
  }
}

class MyProviderCountApp extends StatelessWidget {
  const MyProviderCountApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Provider Counter',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyProviderCounterPage(),
    );
  }
}

class MyProviderCounterPage extends StatelessWidget {
  const MyProviderCounterPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Provider Counter")),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'Number notified by ValueNotifer<int>:',
            ),
            // Consumer<ProviderCounter>(
            //   builder: (context, counter, child) {
            //     return
            Text(
              '${context.watch<ProviderCounter>().count}',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            //   },
            // ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () =>
            // Provider.of<ProviderCounter>(context, listen: false).increment(),
            context.read<ProviderCounter>().increment(),
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
