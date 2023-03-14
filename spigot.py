#! /usr/bin/env python3

# π

def boringWay():
    pi = 3.14159265358979323846264338327950
    return pi

def funWay(n):
    # https://www.cut-the-knot.org/Curriculum/Algorithms/SpigotForPi.shtml

    _al = int(10 * n / 3) + 1
    a = [2] * _al

    held_digits = []
    n_released = 0
    def release_digit(n):
        nonlocal n_released
        if n_released == 1:
            print('.', end='', flush=True)
        print(d, end='', flush=True)
        n_released += 1

    for r in range(0, n):
        #print(r, a)
        for ai in range(0, _al):
            a[ai] *= 10
        for ai in range(_al - 1, 0, -1):
            i = ai + 1
            modulo = 2*i - 1
            q = a[ai] // modulo
            r = a[ai] % modulo
            a[ai] = r
            a[ai - 1] += q * (i - 1)
        q = a[0] // 10
        r = a[0] % 10
        a[0] = r
        
        if q < 9:
            # release held
            for d in held_digits:
                release_digit(d)
            # hold q
            held_digits = [q]
        elif q == 9:
            # add q to held predigits
            held_digits.append(q)
        else:
            assert q == 10
            for d in held_digits:
                # increase by 1 (mod 10)
                d = (d + 1) % 10
                # ...and release
                release_digit(d)
            # hold 0 as predigit
            held_digits = [0]
    for d in held_digits:
        release_digit(d)
    print("\n ≈ π.")

if __name__ == "__main__":
    import sys
    try:
        n = int(sys.argv[1])
    except IndexError:
        n = 3
    funWay(n)
