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


class Integer(Token):
  weight = 0
  def __init__(self, value):
    self.value = value

  def evaluate(self):
    return int(self.value)


class Float(Token):
  weight = 0
  def __init__(self, value):
    self.value = value

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


class Plus(Token):
  weight = 2
  def __init__(self):
    self.value = "+"

  def evaluate(self, left, right):
    return left + right


class Minus(Token):
  weight = 2
  def __init__(self):
    self.value = "-"

  def evaluate(self, left, right):
    return left - right

class EOL(Token):
  weight = -1
  def __init__(self):
    self.value = "EOL"

  def evaluate(self):
    raise Exception("Cannot evaluate token")

class WhiteSpace(Token):
  weight = -1
  def __init__(self):
    self.value = "Space"

  def evaluate(self):
    raise Exception("Cannot evaluate token")
