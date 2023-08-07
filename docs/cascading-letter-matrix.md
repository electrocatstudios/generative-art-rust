# Cascading Letter Matrix

![Example output Cascading Lettering](cascading-lettering.gif)

This example changes the way the fade between frames works, to favour the green spectrum.
We achieve this by creating new consts in place of the old one, inside the frame copy portion 
of the main loop.

```rust
const DECAY: u32 = 4;

// becomes

const RED_DECAY: u32 = 6;
const GREEN_DECAY: u32 = 4;
const BLUE_DECAY: u32 = 5;
```

These consts are used inside the section where the new frame is made - see (branch)[https://github.com/electrocatstudios/generative-art-rust/blob/example/matrix-lettering/src/main.rs] for an example.

We then need to create a struct to hold the lanes information for each lane:

```rust
struct LetterDrop {
    x: i32,
    offset: usize,
    letters: vec::Vec::<String>,
    cur_sel: usize
}
```

During the setup phase of the application (before the main loop) we need to randomly generate the characters based on some new consts.

```rust
    let font = get_font();

    let mut rng = rand::thread_rng();

    let mut letter_drops: vec::Vec::<LetterDrop> = vec::Vec::new();
    let lane_width = (WIDTH - 10)/NUMBER_LANES; // 10 is twice the border width

    for lane_num in 0..NUMBER_LANES {
        let mut vec_letters = vec::Vec::new();
        for i in 0..NUMBER_CHARACTERS_PER_LANE{
            let letter: char = rng.gen_range(b'A'..b'Z') as char;
            let s = format!("{}", letter);
            vec_letters.push(s);
        }
       
        let offset: usize = rng.gen_range(0..NUMBER_CHARACTERS_PER_LANE-1) as usize;
        let mut letter_drop = LetterDrop{x: (lane_num * lane_width) as i32 + 5, offset: offset as usize, letters:vec_letters, cur_sel:0};
        letter_drops.push(letter_drop);
    }
```

Then during the main loop we will cycle through the letters and make sure they draw in the correct location:

```rust 
        for letter_drop in letter_drops.iter_mut() {
            let cur_sel_point = letter_drop.letters.len() as f32 * fraction;
            
            letter_drop.cur_sel = cur_sel_point as usize + letter_drop.offset;
            if letter_drop.cur_sel > letter_drop.letters.len() - 1 {
                letter_drop.cur_sel -= letter_drop.letters.len();
            }
            
            let cycle_pos_fraction = letter_drop.cur_sel  as f32 / letter_drop.letters.len() as f32;
            let y_pos = (HEIGHT as f32 * cycle_pos_fraction) as i32; 
            let letter_height = HEIGHT/letter_drop.letters.len() as u32;
            
            let _ = draw_text_mut(
                    &mut image, 
                    Rgb([255,255,255]), 
                    letter_drop.x, 
                    y_pos, 
                    Scale { x: 32.0, y: 32.0 },
                    &font, 
                    &letter_drop.letters[letter_drop.cur_sel].clone()
                );
        }
```

In the example branch there is a section for adding blur (at time of writing) to create a bloom effect. 
However this code is not working correctly so has been commented out at this time. 