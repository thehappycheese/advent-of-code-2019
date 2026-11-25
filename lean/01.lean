

--set_option trace.Meta.synthInstance true



structure Point where
  x : Nat
  y : Nat

def Point.toString (p:Point): String := s!"[{p.x} {p.y}]"


def op1 (a:Nat):Nat := (a / 3) - 2

theorem op1_decreases_nonzero (a: Nat ) (h: op1 a>0): (op1 a) < a:= by
  unfold op1 at *
  omega


def op2 (a:Nat): Nat :=
  let rec inner (ina:Nat) (sum:Nat):Nat :=
    if h:op1 ina>0 then
      have : op1 ina < ina := op1_decreases_nonzero ina h
      inner (op1 ina) (sum+op1 ina)
    else
      sum
  inner a 0

def main : IO Unit := do
  let x1 <- IO.FS.lines "./input/01.txt"
  let x2 := (x1.filter ( · != "")).map String.toNat!
  let x3 := x2.map op1
  IO.println x3.sum

  let x4 := x2.map op2
  IO.println x4.sum
