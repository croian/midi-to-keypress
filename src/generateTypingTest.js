const args = process.argv.slice(2);
const [letters, sep = " ", maxLength = 4, totalLength = 100] = args;

/**
 * Generates random combinations of letters to use as a typing test
 * @param letters {string} - the letters to use
 * @param sep {string} - the separator to use
 * @param maxLength {number} - the maximum length of a string
 * @param totalLength {number} - the total length of the test
 */
function generateCombos(letters, sep, maxLength, totalLength) {
    const result = [];

    const lettersArray = letters.split("");

    let length = 0;
    while (length < totalLength) {
        const randomLength = Math.floor(Math.random() * maxLength) + 1;
        const random = [];
        for (let i = 0; i < randomLength; i++)
            random.push(lettersArray[Math.floor(Math.random() * lettersArray.length)]);

        result.push(random.join(""));
        length += random.length;
    }

    return result.join(sep);
}

console.log(generateCombos(letters, sep, maxLength, totalLength));
