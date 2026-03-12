import { describe, expect, it } from "vitest";
import { formatDate, formatDateLong, isTypingElement } from "../format";

describe("formatDate", () => {
  it("formats date in short form", () => {
    expect(formatDate("2026-10-15")).toMatch(/Oct.*15/);
    expect(formatDate("2026-10-15")).toMatch(/Wed|Thu|Fri/); // weekday
  });
});

describe("formatDateLong", () => {
  it("formats date in long form with year", () => {
    expect(formatDateLong("2026-10-15")).toContain("2026");
    expect(formatDateLong("2026-10-15")).toMatch(/October.*15/);
  });
});

describe("isTypingElement", () => {
  it("returns false for null", () => {
    expect(isTypingElement(null)).toBe(false);
  });

  it("returns true for input element", () => {
    const el = document.createElement("input");
    expect(isTypingElement(el)).toBe(true);
  });

  it("returns true for textarea", () => {
    const el = document.createElement("textarea");
    expect(isTypingElement(el)).toBe(true);
  });

  it("returns false for div", () => {
    const el = document.createElement("div");
    expect(isTypingElement(el)).toBe(false);
  });
});
