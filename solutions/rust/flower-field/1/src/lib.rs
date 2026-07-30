const OFFSET: [(i8, i8); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1), (0, 1), (1, 1)
];
pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden.iter().enumerate().map(|(y, inner)|
        {
            inner.as_bytes().iter().enumerate().map(|(x, &c)|
                {
                    if c == b'*' {'*'}
                    else {
                        match OFFSET.iter()
                          .map(|&(ox, oy)| {
                                (x as i8 + ox , y as i8 + oy)
                            })
                           .filter_map(|(x, y)|{
                                garden.get(y as usize)
                                      .and_then(|&raw|{
                                            raw.as_bytes().get(x as usize)
                                        })
                                })
                            .filter(|&&c| c == b'*')
                            .count(){
                                0 => ' ',
                                n => (n as u8 + '0' as u8) as char
                            }
                    }
                }
            ).collect()
        }).collect()
}