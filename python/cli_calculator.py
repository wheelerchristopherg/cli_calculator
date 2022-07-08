#!/usr/bin/env python

class Token:
  def __init__(self):
    print("initialize token")

class Number(Token):
  pass

class Multiply(Token):
  pass

class Divide(Token):
  pass

class Add(Token):
  pass

class Subtract(Token):
  pass

class BinNode:
  def __init__(self):
    self.value = None
    self.left = None
    self.right = None

def get_token(potential_token):
  raise Exception("Not Implemented")

def parse_tokens(expression):
  tokens = list()
  for t in expression.split():
    tokens.append(get_token(t))
  return tokens

def build_tree(tokens):
  raise Exception("Not Implemented")

def evaluate_tree(root_node):
  raise Exception("Not Implemented")

def main()
  expression = input("> ")
  tokens = parse_tokens(expression)
  tree = build_tree(tokens)
  evaluate_tree(tree)

