import 'package:counter_riverpod/annotaion_app.dart';
import 'package:counter_riverpod/classic_app.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
  runApp(const ProviderScope(
    // child: MyApp(),
    child: MyRiverpodApp(),
  ));
}
