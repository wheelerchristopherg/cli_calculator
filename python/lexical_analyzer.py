import tokens


DIGIT = "digit"
ALPHA = "alpha"


class UnexpectedCharacter(Exception):
    pass


class NoTokenMatched(Exception):
    pass


class State:
    def __init__(self, matched_token_type=None):
        self.transitions = {}
        self.matched_token_type = matched_token_type

    def add_transition(self, character, state):
        self.transitions.update({character: state})

    def next_state(self, character):
        if character in self.transitions:
            return self.transitions[character]
        elif (char_type := State.get_character_type(character)) in self.transitions:
            return self.transitions[char_type]
        elif self.matched_token_type:
            return self.matched_token_type

        if character:
            raise UnexpectedCharacter(character)
        raise NoTokenMatched()

    @staticmethod
    def get_character_type(character):
        if character.isdigit():
            return DIGIT
        elif character.isalpha():
            return ALPHA
        return None


class TokenParser:
    def __init__(self):
        self.initialize_state_machine()

    def initialize_state_machine(self):
        self.states = []
        self.states.append(State())
        self.states.append(State(tokens.Integer))
        self.states.append(State())
        self.states.append(State(tokens.OpenParen))
        self.states.append(State(tokens.CloseParen))
        self.states.append(State(tokens.Minus))
        self.states.append(State(tokens.Plus))
        self.states.append(State(tokens.Multiply))
        self.states.append(State(tokens.Divide))
        self.states.append(State(tokens.Float))
        self.states.append(State(tokens.EOL))
        self.states.append(State(tokens.WhiteSpace))
        self.states.append(State(tokens.Variable))

        self.states[0].add_transition(DIGIT, self.states[1])
        self.states[0].add_transition(".", self.states[2])
        self.states[0].add_transition("(", self.states[3])
        self.states[0].add_transition(")", self.states[4])
        self.states[0].add_transition("-", self.states[5])
        self.states[0].add_transition("+", self.states[6])
        self.states[0].add_transition("*", self.states[7])
        self.states[0].add_transition("/", self.states[8])
        self.states[0].add_transition("\n", self.states[10])
        self.states[0].add_transition(" ", self.states[11])
        self.states[0].add_transition("\t", self.states[11])
        self.states[0].add_transition(ALPHA, self.states[12])
        self.states[0].add_transition("_", self.states[12])

        self.states[1].add_transition(DIGIT, self.states[1])
        self.states[1].add_transition(".", self.states[2])

        self.states[2].add_transition(DIGIT, self.states[9])

        self.states[9].add_transition(DIGIT, self.states[9])

        self.states[11].add_transition(" ", self.states[11])
        self.states[11].add_transition("\t", self.states[11])

        self.states[12].add_transition(ALPHA, self.states[12])
        self.states[12].add_transition(DIGIT, self.states[12])
        self.states[12].add_transition("_", self.states[12])

    def parse(self, _input):
        text = _input + "\n"
        current_state = self.states[0]
        token_start = 0
        char_pos = 0
        token_list = []

        while char_pos < len(text):
            value = None
            try:
                value = current_state.next_state(text[char_pos])
            except UnexpectedCharacter as e:
                error_position = " " * char_pos
                error_position += "^"
                mono_char_text = text.replace("\t", " ")
                raise UnexpectedCharacter(
                    f"Unexpected character {e.args[0]} at position {char_pos}\n{mono_char_text[:-1]}\n{error_position}"
                )
            except NoTokenMatched as e:
                raise e

            if value is tokens.EOL:
                token_list.append(value())
                break

            if type(value) is type(tokens.Token) and issubclass(value, tokens.Token):
                current_state = self.states[0]
                token_text = text[token_start:char_pos]
                new_token = None
                if issubclass(value, tokens.Number) or issubclass(
                    value, tokens.Variable
                ):
                    new_token = value(token_text)
                else:
                    new_token = value()
                if not isinstance(new_token, tokens.WhiteSpace):
                    token_list.append(new_token)
                token_start = char_pos
            elif isinstance(value, State):
                current_state = value
                char_pos += 1

        return token_list
