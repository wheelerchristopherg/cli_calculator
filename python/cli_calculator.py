#!/usr/bin/env python
import re


class Token:
  def __init__(self):
    self.value = ""

  def __str__(self):
    return self.value

  def __repr__(self):
    return self.__str__()


class Number(Token):
  weight = 0
  def __init__(self, value):
    self.value = value

  @staticmethod
  def is_number(text):
    if re.match(r'^-?\d+(\.\d+)?$', text):
      return True
    return False

  def evaluate(self):
    return float(self.value)


class Multiply(Token):
  weight = 1
  def __init__(self):
    self.value = "*"

  def evaluate(self, left, right):
    return left * right


class Divide(Token):
  weight = 1
  def __init__(self):
    self.value = "/"

  def evaluate(self, left, right):
    return left / right


class Add(Token):
  weight = 2
  def __init__(self):
    self.value = "+"

  def evaluate(self, left, right):
    return left + right


class Subtract(Token):
  weight = 2
  def __init__(self):
    self.value = "-"

  def evaluate(self, left, right):
    return left - right


class BinaryTree:
  def __init__(self, token, left=None, right=None):
    self.token = token
    self.left = left
    self.right = right

  def __str__(self):
    return self.token.__str__()

  def __repr__(self):
    return self.__str__()

  def _check_valid_number_token(self):
    if isinstance(self.token, Number): 
      if self.left or self.right:
        raise Exception("Invalid Expression")
      return self.token.evaluate()
    return None

  def _check_valid_operation(self):
    if not (self.left and self.right and self.token):
      raise Exception(f"Invalid Operation: {self.token}({self.left}, {self.right})")

  def evaluate(self):
    if (valid_number := self._check_valid_number_token()) != None:
      return valid_number
    self._check_valid_operation()
    return self.token.evaluate(self.left.evaluate(), self.right.evaluate())


def token_factory(text, env):
  if text in env:
    return Number(env[text])
  elif Number.is_number(text):
    return Number(text)
  elif text == "*":
    return Multiply()
  elif text == "/":
    return Divide()
  elif text == "+":
    return Add()
  elif text == "-":
    return Subtract()
  raise Exception("Invalid token")


def parse_tokens(expression, env):
  tokens = list()
  for t in expression.split():
    tokens.append(token_factory(t, env))
  return tokens


def find_root(tokens):
  root_index = 0
  for i, t in enumerate(tokens):
    if t.weight >= tokens[root_index].weight:
      root_index = i
  return root_index


def build_tree(tokens):
  if not tokens:
    raise Exception("Invalid Expression")
  elif len(tokens) == 1:
    return BinaryTree(tokens[0])
  root_index = find_root(tokens)
  left_node = build_tree(tokens[:root_index])
  right_node = build_tree(tokens[root_index+1:])
  return BinaryTree(tokens[root_index], left_node, right_node)


def evaluate_tree(tree):
  if not tree:
    raise Exception("Invalid expression")
  if isinstance(tree.token, Number):
    if tree.left or tree.right:
      raise Exception("Invalid expression")
    return tree.token.get_decimal()
  return tree.evaluate()


def build_env_from_history(history):
  return {f"x{i}" : f"{value}" for i, value in enumerate(history)}  


def main():
  history = []
  while True:
    expression = input("> ")
    if expression == "":
      break

    #expression = "123.321 + 112 - 759 * 7 / 3"
    env = build_env_from_history(history)
    try:
      tokens = parse_tokens(expression, env)
      tree = build_tree(tokens)
      result = tree.evaluate()
    except Exception as e:
      print(e)
      continue
    history.append(result)
    print(f"x{len(history)-1} = {result}")


if __name__ == "__main__":
  main()

