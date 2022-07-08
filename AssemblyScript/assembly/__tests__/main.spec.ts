import { get_num, reset, increment } from "../index";
import { context, storage } from "near-sdk-as";

describe("Counter ", () => {
  it("should increment by 5", () => {
    increment(5); // increment by 5
    expect(get_num()).toBe(
      5,
      "counter should be one after a single increment."
    );
  });

  it("get_num is the same as reading from storage", () => {
    expect(storage.getPrimitive<i8>("counter", 0)).toBe(
      get_num(),
      'storage.getPrimitive<i8>("counter", 0) = get_num()'
    );
  });

  it("should be resettable", () => {
    increment(5);
    reset(); // reset to zero
    expect(get_num()).toBe(0, "counter should be zero after it is reset.");
  });

  it("should not overflow", () => {
    storage.set<i8>("counter", 127);
    expect(() => {
      increment(1);
    }).toThrow();
  });
});
