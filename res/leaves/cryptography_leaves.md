
- Cryptology
    - Cryptography
        - Symmetric Cryptography
            - Synonymous: private-key, single-key, secret-key cryptography
            - Advantages
                - High speed due to simple base operations
            - Disadvantages
                - Key management - must be exchanged via secure channel
                - With data networks, many different keys are needed: For $n$ participants $a_k = n \times \frac{n-1}{2}$ keys are needed
            - Basics
                - Encryption and decryption are inverse operations if the same key $K$ is used on both sides: $d_k(y) = d_k(e_k(x)) = x$
                - Secure channel via human courier
            - Key length
                - Relevant if brute-force is the best attack
                - Symmetric keys are much shorter than asymmetric ones
                - 64bit: insecure except for data with short-term value
                - 128bit: long-term security of several decades, unless quantum computers become available
                - 256bit: as above, but probably secure against attacks by quantum computers
            - Block Ciphers
                - Basics
                    - Encrypt data block wise
                    - Common for Internet Applications
                    - Common block lengths
                        - 64bits (e.g. DES)
                        - 128bits (e.g. AES)
                - Substitution Ciphers
                    - Replace each plaintext letter by a fixed other letter
                - Shift Cipher
                    - Synonymous: Caeser Cipher
                    - Encryption: $y = e_k(x) = x + k \space mod \space m$
                    - Decryption: $x = d_k(y) = y - k \space mod \space m$
                    - Disadvantage
                        - Small key space of 26
                - Affine Cipher
                    - Encryption: $y = e_k(x) = a \times x + b \space mod \space m$
                    - Decryption: $x = d_k(y) = a^{-1} \times (y-b) \space mod \space m$
                    - Key $k = (a,b)$, has restriction $gcd(m,a) = 1$
                - (DES) Data Encryption Standard
                    - Was invented via proposals
                    - Data
                        - 56bit key-length
                        - 64bit blocks
                        - 16 rounds
                    - DES Feistel Network
                        - E.g. on 03 page 15
                        - Phases
                            0. Split into $L_0$ and $R_0$
                            1. Initial permutation
                                - $IP()$
                            2. 16 rounds
                                - $L_i=R_{i-1}$
                                - $R_i=L_{i-1} \oplus f(R_{i-1}, k_i)$
                                - Function $f$
                                    1. Expansion Function $E$
                                        - Diffusion
                                        - $E(R_{i-1})$
                                    2. XOR with round key $k_i$
                                    3. S-box substitution
                                        - Confusion
                                    4. Permutation within each round
                            3. Final permutation
                                - $IP^{-1}()$
                        - Key Schedule
                            - Why 56 and not 64 bit?
                                - Parity Bits
                                    - Turn last bit such that the sum of true bits is ungerade
                            - Left Shifts
                                - $C_0=C_{16}$
                                - $D_0=D_{16}$
                    - Decryption
                        - Same algorithm except an inverted round-key-stream
                        - $Round_1^d$ should invert $Round_{16}^e$ such that
                            - $L_1^d=R_{15}^e$
                            - $R_1^d=L_{15}^e$
                        - $L_1^d=R_0^d=L_{16}^e=R_{15}^e$
                        - $R_1^d=L_0^d\oplus f(k_i,R_0^d)=(L_{15}^e\oplus f(k_{16},R_{15}^e)) \oplus f(k_i,R_0^d)$
                        - $R_1^d=L_{15}^e$
                    - Security
            - Stream Ciphers
                - Basics
                    - Encrypt bits individually
                    - Small and fast
                    - Less popular than block ciphers in most domains
                    - Common for Embedded Devices
                    - Security depends on key stream $s_i$
                        - Random: $p(s_i = 0) = p(s_i=1)=0,5$
                        - Must be reproducible by sender and receiver
                    - Modulo-2 additions (XOR)
                        - $x_i,y_i,s_i \in \{0,1\}$
                        - $y_i = e_{s_i}(x_i) = x_i + s_i \space mod \space 2 \equiv x_i \oplus s_i$
                        - $x_i = d_{s_i}(y_i) = y_i + s_i \space mod \space 2 \equiv y_i \oplus s_i$
                - Synchronous Stream Ciphers
                    - $s_i$ depends only on the key (and possibly initialization vector)
                - Asynchronous Stream Ciphers
                    - $s_i$ depends on key and cipher text (feedback line)
                - Unbreakable Ciphers
                    - One Time Pad
                        - Developed by Mauborgne and based on Vernam's stream cipher
                        - Unconditionally secure (can be proven)
                        - Details
                            - Plaintext, ciphertext and key stream consist of individual bits: $x_i,y_i,s_i \in \{0,1\}$
                            - XOR (no shit Sherlock)
                            - Key stream is generated by a TRNG
                            - Key stream only known to legitimate parties
                            - Every $s_i$ only used once
                            - $y_0 \equiv x_0 + s_0 \space mod \space 2$ and so on...
                        - Disadvantage
                            - High resource cost
                            - Problem with the generation of initial short key
        - Asymmetric Cryptography
        - Protocols
    - Cryptanalysis
        - Classical Cryptanalysis
            - Basics
                - Recovering plain text $x$ from cipher text $y$
                - Recovering key $k$ from cipher text $y$
            - Mathematical Analysis
            - Brute-Force Analysis
                - Exhaustive Key Search
                    - Synonymous: Brute-Force Attack
                    - Try every possible substitution table (= key)
                    - Infeasible with high key number
                    - Helpers
                        - Letter Frequency Analysis
                            - Letters have different frequencies in languages
                            - Also effective with letter pairs and triples
            - Attack against single LFSRs
                - $s_i=x_i+y_i \space mod \space 2$
                - $s_{i+m}=\sum^{m-1}_{j=0}p_j \times s_{i+j} \space mod \space 2$
                    - $s_i,p_j \in {0,1}$
                    - $i=0,1,2,...$
                - Oscar could create a set of linear equations:
                - $i=m-1: s_{2m-1} \equiv p_{m-1}\times s_{2m-2} + ... + p_1 \times s_m + p_0 \times s_{m-1} \space mod \space 2$
                - After solving the equations Oscar can reverse-engineer the LFSR and when inserting the seed generate all following bits
        - Implementation Attacks
            - Basics
                - Side-Channel Analysis (e.g. measuring power consumption of processor)
                - Relevant for systems an attacker has physical access to
        - Social Engineering
            - Basics
                - Using trust (e.g. Phishing or Phone calls)
- Modular arithmetic
    - All members of a single equivalence class modulo $m$ behave equivalently in the sense, that any of them yields the same remainder if divided by $m$
    - General rule: Reduce intermediate results as soon as possible
    - $x^e \space mod \space m \neq x^{e \space mod \space m} \space mod \space m$
    - Fractions in modulus calculation
        - Multiplication-inverse is dependent on the module
        - E.g.: $\frac{1}{4} \space mod \space 13 \equiv 1 \times 4^{-1} \space mod \space 13 \equiv 1 \times 10 \space mod \space 13 \equiv 10$
            - Iterate until $1$ so that the result is the multiplication-inverse
    - Modular reduction
        - E.g.: $x=3^{10} \space mod \space 10 \equiv (3^3)^{3} \times 3 \space mod \space 13 \equiv 1^3 \times 3 \space mod \space 13 \equiv 3$
    - Trial and error at $n^x \space mod \space m$
    - Integer Ring
        - An integer Ring $\mathbb{Z}_m$ consists of:
            1. The set $\mathbb{Z}_m=\{0,1,2,...,m-1\}$
            2. The operations "$+$" and "$\times$" for all $a,b \in \mathbb{Z}_m$ such that:
                1. $a+b=c \space mod \space m$, with $c \in \mathbb{Z}_m$
                2. $a \times b = d \space mod \space m$, with $d \in \mathbb{Z}_m$
        - Properties
            1. Ring is closed: The result of any addition or multiplication always is an element of the ring
            2. Associativity
            3. Neutral Element of addition
                - $a+0 \equiv a \space mod \space m$
            4. Neutral Element of multiplication
                - $a \times 1 \equiv a \space mod \space m$
            5. Inverse element addition
                - $a + (-a) \equiv 0 \space mod \space m$
            6. Inverse element multiplication
                - For some elements
                - $a \times a^{-1} \equiv 1 \space mod \space m$
            7. Distributivity
    - Fields (germ: Gruppe)
        - Example given
            - The set $\mathbb{Z}_5=\{0,1,2,3,4\}$
            - G: $(\mathbb{Z}_5,\circ)$, $\circ$ means "Operation"
            - Only view it in relation to either Addition or Multiplikation, not both at the same time
        - Properties
            1. Isolation
            2. Associativity
            3. Neutral element
            4. Inverse Element
            5. Commutativity (then it is called a commutative Group)
            6. Distributivity (if the two group operations are mixed)
    - Prime Fields
        - Theorem: Let $p$ be a prime number. The set of elements $\mathbb{Z}_p=\{0,1,2,,...,p-1\}$ is denoted as $GF(p)$ and is referred to as a Prime Field or Galois Field
        - $GF(2)$ is the smallest finite field
            - Uses $mod \space 2$
            - Addition equal to XOR
            - Multiplication equal to AND
        - todo!("Siehe Folie 2a")
        - $a\cdot a^{-1} \space mod \space p \equiv 1$
        - E.g.:
            - $GF(6)$, where $6$ is not a prime number, has no multip. inverse for every number and therefore is not a Galois Filed
            - $GF(7)$, where $7$ is a prime number, has a multip. inverse and is a Galois Field
    - Primitive Element
        - todo!("Leck Eier. Siehe Folie 2a")
        - Theorem: Every GF does contain at least one Primitive Field
    - Extension Fields
        - todo!("Siehe Folie 2a")
        - Each element of $GF(p^m)$ is a polynomial with coefficients over $GF(p)$
        - Irreducible polynomial
        - E.g.:
            - $A(x)=a_7x^7+a_6x^6+a_5x^5...a_0x^0$
            - $A=(a_7a_6a_5...a_0)$
    - Irreducible Polynomials
        - todo!()
    - Primitive Polynomials
        - todo!()
        - Goal: create some sort of complex solution (like $i$ in normal math) for irreducible polynomials it is called $\alpha$
        - E.g.:
            - $GF(2^4): p(x)=a_3x^3+a_2...$
            - $p(x)=x^4+x+1$ has no solution
            - $p(\alpha)=\alpha^4+\alpha+1\equiv 0$ has a solution ($+\alpha + 1$)
            - $\alpha^4+2\alpha +2=\alpha +1$ ($mod \space 2$)
            - $\alpha^4=\alpha +1$
- (RNG) Random Number Generators
    - (TRNG) True Random Number Generators
        - E.g. flipping a coin
        - Often needed in cryptography for generating session keys
        - The same sequence _cannot be reproduced_
    - (PRNG) Pseudorandom Number Generators
        - Generate sequence from initial seed value
        - Typically good statistical properties: $\frac{1}{2}^k$
        - The same sequence _can be reproduced and be predicted_
        - $s_0 = seed$
        - $s_{i+1} = f(s_i,s_{i-1},s_{i-2},...,s_{i-t})$
        - Most PRNGs have bad cryptographic properties
            - If some bits of plaintext are known (e.g. file header), the key (A,B) can be calculated
            - Therefor CSPRNG-notation was invented
        - (LFSR) Linear Feedback Shift Registers
            - Are used for long Pseudo Random Number Sequences with good statistical properties
            - Cryptographically not that usable because of predictability of the linear shift-registers
                - Combination of multiple shift-registers is used
            - Flip-Flops
            - $s_{i+3} = s_{i+1}+s_i \space mod \space 2$
            - Maximum output length is calculated with $L = 2^m-1$
                - $m$ being the number of storage elements
                - $L$ being the sequence length
            - Full $0$-rows are left out because otherwise the $\oplus$ would remain in 0-mode
    - (CSPRNG) Cryptographically secure PRNGs
        - Unpredictable: Given a sequence of $n$ bits of the key stream, it is not possible to compute the following bits
        - Needed in particular for stream ciphers
        - Need for unpredictability is unique to cryptography
        - Utilizing a combination of several LFSRs with extra nonlinear components
        - Example A5/1-Stream Cipher (used in GSM)
            - Light-weight
            - Has multiple phases with different concepts
                1. Initialization
                2. 22 cycles to add frame number
                3. Warm-up phase
                4. Key stream generation phase
            - Irregular Clocking is the key concept, but it is still linear
            - Clocking bits are used for "majority-rule"
            - Was reverse engineered and can now be broken in short time
            - [https://youtu.be/LgZAI3DdUA4?si=npmfIsh5Ex3DmE8C]
        - Example Trivium
            - Light-weight
            - Three phases
                1. Initialization
                    - Initialization vector (IV) for different streams from same key
                    - New IV for each encryption session
                2. Warm-up phase
                3. Encryption phase
            - Three nonlinear FSRs
            - Arranged in circle-like fashion, i.e. output of one register connected to input of another
            - Three AND-gates which get rid of the linearity of the system
                - Cannot be traced back by Oscar
- Theory of numbers
    - "Mathematics is the Queen of Science - Number Theory is the Queen of Mathematics." ~Gauss
    - Primes Fields
    - Extension Fields $GF(2^m)$
        - Synonymous: Finite Fields
        - Galois Fields
            - $GF(2)={0,1}$
- General stuff
    - Third party monitoring or archiving of data over media can **never** be avoided
    - Data manipulation (but it can be detected)
    - A encryption has to be secure against all possible attacks
    - Kerkhoff's Principle
        - "A cryptosystem should be secure even if the attacker  knows all details about the system, with the exception of the secret key. In particular, the system should be secure when the attacker knows the encryption and decryption algorithms."
    - Weakest link in the chain will be used
    - A cryptosystem is _unconditionally secure_ if it cannot be broken even with infinite computational resources.
    - A cryptosystem is _computationally secure_ if the best known algorithm for breaking it requires at least $t$ operations.
    - Confusion and Diffusion
        - Confusion
            - Relationship between key and ciphertext is obscured
        - Diffusion
            - Influence of one plaintext symbol is spread over many ciphertext symbols
- Security types in information exchange
    - Commercial security
        - Manipulation of sender, receiver, account numbers, money amounts
        - Monitoring of data
    - Privacy
        - Privacy reasons
        - Cost control
        - Authentication 
