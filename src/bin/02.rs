use derive_more::{Error, Display};
use std::{path::Path};

use advent_of_code_2019::parse::read_input;


#[derive(Debug, Display)]
enum PuterErrorKind{
    OutOfBoundsAccess,
    OutOfBoundsWrite,
    UnknownInstruction,
}
#[derive(Debug, Display, Error)]
#[display("PuterError({} {} {})","self.kind","self.instruction_pointer","self.instruction_counter")]
struct PuterError{
    kind:PuterErrorKind,
    message:&'static str,
    instruction_pointer:usize,
    instruction_counter:usize,
}


#[derive(Debug)]
enum ComputerState {
    Continue,
    Halt,
}

#[derive(Debug, Clone)]
struct Puter {
    memory: Vec<i64>,
    instruction_pointer: usize,
    instruction_counter: usize
}

impl Puter {
    fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            instruction_pointer: 0,
            instruction_counter: 0,
        }
    }
    fn read(&self, address:i64)-> Result<i64,PuterError>{
        if address<0 || (address as usize) >= self.memory.len(){
            return Err(self.error(PuterErrorKind::OutOfBoundsAccess, "out of bounds"))
        }
        unsafe{
            Ok(*self.memory.get_unchecked(address as usize))
        }
    }
    fn write(&mut self, address:i64, value:i64)-> Result<(),PuterError>{
        if address <0 || (address as usize) >= self.memory.len(){
            return Err(self.error(PuterErrorKind::OutOfBoundsWrite, "out of bounds "))
        }
        unsafe{
            *self.memory.get_unchecked_mut(address as usize) = value;
        }
        Ok(())
    }
    fn add_store(&mut self, a: i64, b: i64, store: i64) -> Result<(), PuterError>{
        let a = self.read(a)?;
        let b = self.read(b)?;
        self.write(store,  a + b)
    }
    fn mul_store(&mut self, a: i64, b: i64, store: i64) -> Result<(), PuterError> {
        let a = self.read(a)?;
        let b = self.read(b)?;
        self.write(store,  a * b)
    }
    fn error(&self, kind:PuterErrorKind, message:&'static str)->PuterError{
        PuterError { kind, message, instruction_pointer: self.instruction_pointer, instruction_counter: self.instruction_counter }
    }
    fn step(&mut self) -> Result<ComputerState, PuterError> {
        if self.instruction_pointer>=self.memory.len(){
            return Err(self.error(PuterErrorKind::OutOfBoundsAccess, "instruction pointer overflow"))
        }
        match self.memory[self.instruction_pointer] {
            1 => {
                self.add_store(
                    self.read(self.instruction_pointer as i64 + 1)?,
                    self.read(self.instruction_pointer as i64 + 2)?,
                    self.read(self.instruction_pointer as i64 + 3)?,
                )?;
                self.instruction_counter+=1;
                self.instruction_pointer += 4;
                Ok(ComputerState::Continue)
            }
            2 => {
                self.mul_store(
                    self.read(self.instruction_pointer as i64 + 1)?,
                    self.read(self.instruction_pointer  as i64+ 2)?,
                    self.read(self.instruction_pointer as i64 + 3)?,
                )?;
                self.instruction_counter+=1;
                self.instruction_pointer += 4;
                Ok(ComputerState::Continue)
            }
            99 => Ok(ComputerState::Halt),
            _ => Err(self.error(PuterErrorKind::UnknownInstruction, "Unknown instruction")),
        }
        
    }
    fn run(&mut self) -> Result<i64, PuterError> {
        loop {
            match self.step() {
                Ok(ComputerState::Continue) => continue,
                Ok(ComputerState::Halt) => return Ok(self.memory[0]),
                Err(pe)=> return Err(pe)
            }
        }
    }
}

fn part1(mut input: Vec<i64>, noun: i64, verb: i64) -> Result<i64, PuterError> {
    input[1] = noun;
    input[2] = verb;
    let mut p = Puter::new(input);
    p.run()
}

fn part2(input: Vec<i64>) -> Result<(i64,i64), ()> {
    for noun in 0..100i64{
        for verb in 0..100i64{
            let res = part1(input.clone(), noun, verb);
            match res{
                Ok(v) if v==19690720 =>{
                    return Ok((noun, verb))
                }
                _=>continue
            }
        }
    }
    Err(())
}

fn main() {
    let input: Vec<i64> = read_input(Path::new("./input/02.txt"))
        .split(",")
        .map(|i| str::parse(&i).expect("unexpected input"))
        .collect();

    println!("Part 1 {:?}", part1(input.clone(), 12, 2));
    println!("Part 2 {:?}", part2(input.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input: Vec<i64> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut p = Puter::new(input);
        
        let result = p.run().unwrap();
        assert_eq!(result, 3500)
    }
}
