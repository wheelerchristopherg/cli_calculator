import binary_tree


class Token:
    def __init__(self):
        self.value = ""

    def __str__(self):
        return self.value

    def __repr__(self):
        return self.__str__()


class Paren(Token):
    def __init(self):
        self.value = ""


class OpenParen(Paren):
    weight = 0

    def __init__(self):
        self.value = "("


class CloseParen(Paren):
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
            if isinstance(result, binary_tree.BinaryTree):
                return result.evaluate(env)
            return result
        raise Exception("Unknown Variable: {}".format(self.value))


class Number(Token):
    weight = -1

    def __init__(self, value):
        self.value = value

    def evaluate(self):
        raise Exception("Cannot evaluate token")


class Integer(Number):
    weight = 0

    def __init__(self, value):
        self.value = value

    def evaluate(self):
        return int(self.value)


class Float(Number):
    weight = 0

    def __init__(self, value):
        self.value = value

    def evaluate(self):
        return float(self.value)


class Operator(Token):
    weight = -1

    def __init__(self):
        self.value = "*"

    def evaluate(self, left, right):
        return left * right


class Multiply(Operator):
    weight = 1

    def __init__(self):
        self.value = "*"

    def evaluate(self, left, right):
        return left * right


class Divide(Operator):
    weight = 1

    def __init__(self):
        self.value = "/"

    def evaluate(self, left, right):
        try:
            return left / right
        except:
            raise Exception("Divide by Zero")


class Plus(Operator):
    weight = 2

    def __init__(self):
        self.value = "+"

    def evaluate(self, left, right):
        return left + right


class Minus(Operator):
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
