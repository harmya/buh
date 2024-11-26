use crate::{operator::Operator, utils::extract_whitespace, val::Val, Number};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(Number),
    Operation {
        first_operand: Number,
        second_operand: Number,
        operator: Operator,
    },
}

impl Expression {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (first_operand, rest) = Number::new(s.trim());
        let (_, rest) = extract_whitespace(rest);

        let (operator, rest) = Operator::new(rest.trim());
        let (_, rest) = extract_whitespace(rest);

        let (second_operand, rest) = Number::new(rest.trim());
        Ok((
            Self {
                first_operand,
                second_operand,
                operator,
            },
            rest,
        ))
    }

    pub(crate) fn eval(&self) -> Val {
        let Number(first_operand) = self.first_operand;
        let Number(second_operand) = self.second_operand;

        let result = match self.operator {
            Operator::Add => first_operand + second_operand,
            Operator::Sub => first_operand - second_operand,
            Operator::Mul => first_operand * second_operand,
            Operator::Div => first_operand / second_operand,
            Operator::Mod => first_operand % second_operand,
            _ => panic!("Illegal Operator: {:?}", self.operator),
        };

        Val::Number(result)
    }
}
