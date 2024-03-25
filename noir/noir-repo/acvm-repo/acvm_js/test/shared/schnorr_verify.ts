// See `schnorr_verify_circuit` integration test in `acir/tests/test_program_serialization.rs`.
export const bytecode = Uint8Array.from([
  31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 77, 210, 7, 78, 2, 1, 20, 69, 81, 236, 189, 247, 222,
  123, 239, 93, 177, 33, 34, 238, 194, 253, 47, 193, 200, 147, 67, 194, 36, 147, 163, 33, 33,
  228, 191, 219, 82, 168, 63, 63, 181, 183, 197, 223, 177, 147, 191, 181, 183, 149, 69, 159,
  183, 213, 222, 238, 218, 219, 206, 14, 118, 178, 139, 141, 183, 135, 189, 236, 99, 63, 7,
  56, 200, 33, 14, 115, 132, 163, 28, 227, 56, 39, 56, 201, 41, 78, 115, 134, 179, 156, 227,
  60, 23, 184, 200, 37, 46, 115, 133, 171, 92, 227, 58, 55, 184, 201, 45, 110, 115, 135, 187,
  220, 227, 62, 15, 120, 200, 35, 30, 243, 132, 167, 60, 227, 57, 47, 120, 201, 43, 94, 243,
  134, 183, 188, 227, 61, 31, 248, 200, 39, 22, 249, 204, 151, 166, 29, 243, 188, 250, 255,
  141, 239, 44, 241, 131, 101, 126, 178, 194, 47, 86, 249, 237, 123, 171, 76, 127, 105, 47,
  189, 165, 181, 116, 150, 198, 26, 125, 245, 248, 45, 233, 41, 45, 165, 163, 52, 148, 126,
  210, 78, 186, 73, 51, 233, 37, 173, 164, 147, 52, 146, 62, 210, 70, 186, 72, 19, 233, 33,
  45, 164, 131, 52, 144, 253, 151, 11, 245, 221, 179, 121, 246, 206, 214, 217, 57, 27, 103,
  223, 109, 187, 238, 218, 115, 223, 142, 135, 246, 59, 182, 219, 169, 189, 206, 237, 116,
  105, 159, 107, 187, 220, 218, 227, 222, 14, 143, 238, 95, 116, 247, 23, 119, 126, 115, 223,
  146, 187, 150, 221, 179, 226, 142, 141, 155, 53, 238, 86, 104, 186, 231, 255, 243, 7, 100,
  141, 232, 192, 233, 3, 0, 0,
]);

export const initialWitnessMap = new Map([
  [1, '0x04b260954662e97f00cab9adb773a259097f7a274b83b113532bce27fa3fb96a'],
  [2, '0x2fd51571db6c08666b0edfbfbc57d432068bccd0110a39b166ab243da0037197'],
  [3, '0x000000000000000000000000000000000000000000000000000000000000002e'],
  [4, '0x00000000000000000000000000000000000000000000000000000000000000ce'],
  [5, '0x0000000000000000000000000000000000000000000000000000000000000052'],
  [6, '0x00000000000000000000000000000000000000000000000000000000000000aa'],
  [7, '0x0000000000000000000000000000000000000000000000000000000000000087'],
  [8, '0x000000000000000000000000000000000000000000000000000000000000002a'],
  [9, '0x0000000000000000000000000000000000000000000000000000000000000049'],
  [10, '0x000000000000000000000000000000000000000000000000000000000000009d'],
  [11, '0x0000000000000000000000000000000000000000000000000000000000000050'],
  [12, '0x000000000000000000000000000000000000000000000000000000000000007c'],
  [13, '0x000000000000000000000000000000000000000000000000000000000000009a'],
  [14, '0x00000000000000000000000000000000000000000000000000000000000000aa'],
  [15, '0x00000000000000000000000000000000000000000000000000000000000000df'],
  [16, '0x0000000000000000000000000000000000000000000000000000000000000023'],
  [17, '0x0000000000000000000000000000000000000000000000000000000000000034'],
  [18, '0x0000000000000000000000000000000000000000000000000000000000000010'],
  [19, '0x000000000000000000000000000000000000000000000000000000000000008a'],
  [20, '0x0000000000000000000000000000000000000000000000000000000000000047'],
  [21, '0x0000000000000000000000000000000000000000000000000000000000000063'],
  [22, '0x00000000000000000000000000000000000000000000000000000000000000e8'],
  [23, '0x0000000000000000000000000000000000000000000000000000000000000037'],
  [24, '0x0000000000000000000000000000000000000000000000000000000000000054'],
  [25, '0x0000000000000000000000000000000000000000000000000000000000000096'],
  [26, '0x000000000000000000000000000000000000000000000000000000000000003e'],
  [27, '0x00000000000000000000000000000000000000000000000000000000000000d5'],
  [28, '0x00000000000000000000000000000000000000000000000000000000000000ae'],
  [29, '0x0000000000000000000000000000000000000000000000000000000000000024'],
  [30, '0x000000000000000000000000000000000000000000000000000000000000002d'],
  [31, '0x0000000000000000000000000000000000000000000000000000000000000020'],
  [32, '0x0000000000000000000000000000000000000000000000000000000000000080'],
  [33, '0x000000000000000000000000000000000000000000000000000000000000004d'],
  [34, '0x0000000000000000000000000000000000000000000000000000000000000047'],
  [35, '0x00000000000000000000000000000000000000000000000000000000000000a5'],
  [36, '0x00000000000000000000000000000000000000000000000000000000000000bb'],
  [37, '0x00000000000000000000000000000000000000000000000000000000000000f6'],
  [38, '0x00000000000000000000000000000000000000000000000000000000000000c3'],
  [39, '0x000000000000000000000000000000000000000000000000000000000000000b'],
  [40, '0x000000000000000000000000000000000000000000000000000000000000003b'],
  [41, '0x0000000000000000000000000000000000000000000000000000000000000065'],
  [42, '0x00000000000000000000000000000000000000000000000000000000000000c9'],
  [43, '0x0000000000000000000000000000000000000000000000000000000000000001'],
  [44, '0x0000000000000000000000000000000000000000000000000000000000000085'],
  [45, '0x0000000000000000000000000000000000000000000000000000000000000006'],
  [46, '0x000000000000000000000000000000000000000000000000000000000000009e'],
  [47, '0x000000000000000000000000000000000000000000000000000000000000002f'],
  [48, '0x0000000000000000000000000000000000000000000000000000000000000010'],
  [49, '0x00000000000000000000000000000000000000000000000000000000000000e6'],
  [50, '0x0000000000000000000000000000000000000000000000000000000000000030'],
  [51, '0x000000000000000000000000000000000000000000000000000000000000004a'],
  [52, '0x0000000000000000000000000000000000000000000000000000000000000018'],
  [53, '0x000000000000000000000000000000000000000000000000000000000000007c'],
  [54, '0x00000000000000000000000000000000000000000000000000000000000000d0'],
  [55, '0x00000000000000000000000000000000000000000000000000000000000000ab'],
  [56, '0x0000000000000000000000000000000000000000000000000000000000000031'],
  [57, '0x00000000000000000000000000000000000000000000000000000000000000d5'],
  [58, '0x0000000000000000000000000000000000000000000000000000000000000063'],
  [59, '0x0000000000000000000000000000000000000000000000000000000000000084'],
  [60, '0x00000000000000000000000000000000000000000000000000000000000000a3'],
  [61, '0x00000000000000000000000000000000000000000000000000000000000000a6'],
  [62, '0x00000000000000000000000000000000000000000000000000000000000000d5'],
  [63, '0x0000000000000000000000000000000000000000000000000000000000000091'],
  [64, '0x000000000000000000000000000000000000000000000000000000000000000d'],
  [65, '0x000000000000000000000000000000000000000000000000000000000000009c'],
  [66, '0x00000000000000000000000000000000000000000000000000000000000000f9'],
  [67, '0x0000000000000000000000000000000000000000000000000000000000000000'],
  [68, '0x0000000000000000000000000000000000000000000000000000000000000001'],
  [69, '0x0000000000000000000000000000000000000000000000000000000000000002'],
  [70, '0x0000000000000000000000000000000000000000000000000000000000000003'],
  [71, '0x0000000000000000000000000000000000000000000000000000000000000004'],
  [72, '0x0000000000000000000000000000000000000000000000000000000000000005'],
  [73, '0x0000000000000000000000000000000000000000000000000000000000000006'],
  [74, '0x0000000000000000000000000000000000000000000000000000000000000007'],
  [75, '0x0000000000000000000000000000000000000000000000000000000000000008'],
  [76, '0x0000000000000000000000000000000000000000000000000000000000000009'],
]);

export const expectedWitnessMap = new Map(initialWitnessMap).set(
  77,
  '0x0000000000000000000000000000000000000000000000000000000000000001',
);