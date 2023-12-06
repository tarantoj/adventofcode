use nom::bytes::complete::take;
use nom::number::complete::be_u16;
use nom::IResult;

pub fn length_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, length) = be_u16(input)?;
    let game = tag("Game");
    take(length)(input)
}
