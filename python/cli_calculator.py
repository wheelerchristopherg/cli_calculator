#!/usr/bin/env python
import re
import tokens
import lexical_analyzer
import binary_tree


def build_env_from_history(history):
  return {f"x{i}" : value for i, value in enumerate(history)}  


def main():
  history = []
  parser = lexical_analyzer.TokenParser()
  while True:
    try:
      expression = input("> ")
      if expression == "":
        break

      env = build_env_from_history(history)
      tokens = parser.parse(expression)
      tree_builder = binary_tree.TreeBuilder()
      tree_builder.build_tree(tokens)
      tree = tree_builder.get_tree()
      env.update(tree_builder.get_env())
      result = tree.evaluate(env)
    except Exception as e:
      print(e)
      continue
    history.append(result)
    print(f"x{len(history)-1} = {result}")


if __name__ == "__main__":
  main()

