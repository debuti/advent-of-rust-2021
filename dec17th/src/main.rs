/**
*   This puzzle a mathematical challenge, the 1st part can be easily solved without coding anything.
*
* 1st part)

*   If you realize that when the probe goes up by x distance it will go down by the same
* distance, then its easier. The probe will always pass through zero. Then, the higher
* height will correspond with the higher jump from zero to the lowest part of the target.
* This means that for the example it will jump 0 - -10 = 10. Then to get the highest y,
* we will make n*(n-1)/2, so in the example case it will be 10*9/2 = 45
*
* 2nd part)
*
*   We will need to count the different possibilities for each axis, lets use a computer for this
*
*  x)
*   For the x axis we have the chance to hit the target in just one jump (reaching it at
* maximum x velocity), so this means we will have 30-20 = 10 different initial velocities.
*   Then we can hit the target (reaching it with velocity zero), this will comply with
* n*(n-1)/2 = x so x belongs to [20..30]
* according to WolframAlpha (https://www.wolframalpha.com/input/?i=20%3C%3Dn%28n-1%29%2F2%3C%3D30)
* this is 7 or 8.
*   Anything under 7 wont reach the target, and anything over 30 will pass over.
*
*  y)
*   For y we have two types of shots, the positive and the negative ones:
*  - The negative shots are kind alike to x: You can either shot them directly with one step
* (maximum velocity), or shot them at zero speed. Everything in between will have to be calculated.
* Upper bound = 0m/s - n(n-1)/2 steps, Lower bound = lowest target m/s - 1 step
* In the example 5<n(n+1)/2<10 yields 3 steps for the upper bound
* and 10m/s for the lower bound
*  - As every positive shot will turn into a negative one after a given number of steps, we can think
* of them as they were negative shots, but with a twist
* Upper bound = -1 * lowest target m/s - n(n-1) steps, Lower bound = 0m/s - n(n-1)/2 steps
*
*/

fn main() {
    let _target: ((i32, i32), (i32, i32)) = ((20, 30), (-5, -10));
    let target: ((i32, i32), (i32, i32)) = ((32, 65), (-177, -225));

    let mut count = 0;
    for vx0 in 0i32..=target.0 .1 {
        for vy0 in target.1 .1..=target.1 .1.abs() {
            let mut s = (0, 0);
            let mut v = (vx0, vy0);
            while s.1 >= target.1 .1 {
                s.0 += v.0;
                if v.0 > 0 {
                    v.0 -= 1;
                }
                s.1 += v.1;
                v.1 -= 1;
                if (target.0 .0 <= s.0 && s.0 <= target.0 .1)
                    && (target.1 .0 >= s.1 && s.1 >= target.1 .1)
                {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("2: {}", count);
}
