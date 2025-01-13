pub trait CheckedDecrement
where
    Self: Sized,
{
    fn checked_decrement(&self) -> Option<Self>;
}

impl CheckedDecrement for usize {
    fn checked_decrement(&self) -> Option<Self> {
        self.checked_sub(1)
    }
}

impl CheckedDecrement for isize {
    fn checked_decrement(&self) -> Option<Self> {
        self.checked_sub(1)
    }
}

impl CheckedDecrement for i32 {
    fn checked_decrement(&self) -> Option<Self> {
        self.checked_sub(1)
    }
}

impl CheckedDecrement for u32 {
    fn checked_decrement(&self) -> Option<Self> {
        self.checked_sub(1)
    }
}

impl CheckedDecrement for i64 {
    fn checked_decrement(&self) -> Option<Self> {
        self.checked_sub(1)
    }
}

impl CheckedDecrement for u64 {
    fn checked_decrement(&self) -> Option<Self> {
        self.checked_sub(1)
    }
}

pub trait CheckedIncrement
where
    Self: Sized,
{
    fn checked_increment(&self) -> Option<Self>;
}

impl CheckedIncrement for usize {
    fn checked_increment(&self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl CheckedIncrement for isize {
    fn checked_increment(&self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl CheckedIncrement for i32 {
    fn checked_increment(&self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl CheckedIncrement for u32 {
    fn checked_increment(&self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl CheckedIncrement for i64 {
    fn checked_increment(&self) -> Option<Self> {
        self.checked_add(1)
    }
}

impl CheckedIncrement for u64 {
    fn checked_increment(&self) -> Option<Self> {
        self.checked_add(1)
    }
}

pub trait CheckedSub
where
    Self: Sized,
{
    fn checked_sub(&self, rhs: Self) -> Option<Self>;
}

impl CheckedSub for isize {
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        isize::checked_sub(*self, rhs)
    }
}

impl CheckedSub for usize {
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        usize::checked_sub(*self, rhs)
    }
}

impl CheckedSub for i32 {
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        i32::checked_sub(*self, rhs)
    }
}

impl CheckedSub for u32 {
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        u32::checked_sub(*self, rhs)
    }
}

impl CheckedSub for i64 {
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        i64::checked_sub(*self, rhs)
    }
}

impl CheckedSub for u64 {
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        u64::checked_sub(*self, rhs)
    }
}

pub trait CheckedAdd
where
    Self: Sized,
{
    fn checked_add(&self, rhs: Self) -> Option<Self>;
}

impl CheckedAdd for isize {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        isize::checked_add(*self, rhs)
    }
}

impl CheckedAdd for usize {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        usize::checked_add(*self, rhs)
    }
}

impl CheckedAdd for i32 {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        i32::checked_add(*self, rhs)
    }
}

impl CheckedAdd for u32 {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        u32::checked_add(*self, rhs)
    }
}

impl CheckedAdd for i64 {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        i64::checked_add(*self, rhs)
    }
}

impl CheckedAdd for u64 {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        u64::checked_add(*self, rhs)
    }
}

pub trait LessThanZero {
    fn less_than_zero(&self) -> bool;
}

impl LessThanZero for usize {
    fn less_than_zero(&self) -> bool {
        false
    }
}

impl LessThanZero for u32 {
    fn less_than_zero(&self) -> bool {
        false
    }
}

impl LessThanZero for u64 {
    fn less_than_zero(&self) -> bool {
        false
    }
}

impl LessThanZero for isize {
    fn less_than_zero(&self) -> bool {
        self < &(0 as isize)
    }
}

impl LessThanZero for i32 {
    fn less_than_zero(&self) -> bool {
        self < &(0 as i32)
    }
}

impl LessThanZero for i64 {
    fn less_than_zero(&self) -> bool {
        self < &(0 as i64)
    }
}
