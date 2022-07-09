#!/usr/bin/env python
import re


class Token:
  def __init__(self):
    self.value = ""

  def __str__(self):
    return self.value

  def __repr__(self):
    return self.__str__()


class OpenParen(Token):
  weight = 0
  def __init__(self):
    self.value = "("


class CloseParen(Token):
  weight = 0
  def __init__(self):
    self.value = ")"


class Variable(Token):
  weight = 0
  def __init__(self, value):
    self.value = value

  def evaluate(self, env):
    if self.value in env:
      result = env[self.value]
      if isinstance(result, BinaryTree):
        return result.evaluate(env)
      return result
    raise Exception(f"Invalid Variable: {self.value}")


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
        raise Exception("Invalid Tree")
      return self.token.evaluate()
    return None

  def _check_valid_var_token(self, env):
    if isinstance(self.token, Variable): 
      if self.left or self.right:
        raise Exception("Invalid Tree")
      return self.token.evaluate(env)
    return None

  def _check_valid_operation(self):
    if not (self.left and self.right and self.token):
      raise Exception(f"Invalid Operation: {self.token}({self.left}, {self.right})")

  def evaluate(self, env):
    if (valid_number := self._check_valid_number_token()) != None:
      return valid_number

    if (resolved_var := self._check_valid_var_token(env)) != None:
        return resolved_var

    self._check_valid_operation()
    return self.token.evaluate(self.left.evaluate(env), self.right.evaluate(env))


class TreeBuilder:
  def __init__(self, tokens):
    self.sub_tree_map = {}
    self.tokens = tokens

  def find_root(self, tokens):
    root_index = 0
    for i, t in enumerate(tokens):
      if t.weight >= tokens[root_index].weight:
        root_index = i
    return root_index
  
  
  def build_tree(self, tokens):
    if not tokens:
      raise Exception("Invalid Expression")
    elif len(tokens) == 1:
      return BinaryTree(tokens[0])
    root_index = find_root(tokens)
    left_node = build_tree(tokens[:root_index])
    right_node = build_tree(tokens[root_index+1:])
    return BinaryTree(tokens[root_index], left_node, right_node)


def token_factory(text):
  if Number.is_number(text):
    return Number(text)
  elif text == "*":
    return Multiply()
  elif text == "/":
    return Divide()
  elif text == "+":
    return Add()
  elif text == "-":
    return Subtract()
  elif text == "(":
    return OpenParen()
  elif text == ")":
    return CloseParen()
  elif text != "":
    return Variable(text)
  raise Exception("Invalid token")


def parse_tokens(expression):
  tokens = list()
  for t in expression.split():
    tokens.append(token_factory(t))
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


def build_env_from_history(history):
  return {f"x{i}" : value for i, value in enumerate(history)}  


def main():
  history = []
  while True:
    expression = input("> ")
    if expression == "":
      break

    #expression = "123.321 + 112 - 759 * 7 / 3"
    env = build_env_from_history(history)
    try:
      tokens = parse_tokens(expression)
      tree = build_tree(tokens)
      result = tree.evaluate(env)
    except Exception as e:
      print(e)
      continue
    history.append(result)
    print(f"x{len(history)-1} = {result}")


if __name__ == "__main__":
  main()

