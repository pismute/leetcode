export function deepEqual<T extends any>(a: T, b: T): boolean {
  if (a === b) { return true; }


  if (a && b && typeof a === 'object' && typeof b === 'object') {
    const arrA = Array.isArray(a);
    const arrB = Array.isArray(b);

    if (arrA && arrB) {
      let length = a.length;
      if (length !== b.length) { return false; }
      for (let i = length; i-- !== 0;) {
        if (!deepEqual(a[i], b[i])) {
          return false;
        }
      }
      return true;
    }

    if (arrA !== arrB) { return false; }

    const dateA = a instanceof Date
      , dateB = b instanceof Date;
    if (dateA !== dateB) { return false; }
    if (dateA && dateB) { return a.getTime() === b.getTime(); }

    const regexpA = a instanceof RegExp
      , regexpB = b instanceof RegExp;
    if (regexpA !== regexpB) { return false; }
    if (regexpA && regexpB) { return a.toString() === b.toString(); }

    const keys: string[] = Object.keys(a);
    const length = keys.length;

    if (length !== Object.keys(b).length) {
      return false;
    }

    for (let i = length; i-- !== 0;) {
      if (!Object.prototype.hasOwnProperty.call(b, keys[i] as PropertyKey)) {
        return false;
      }
    }

    for (let i = length; i-- !== 0;) {
      let key = keys[i] as string;
      if (!deepEqual(a[key as keyof typeof a], b[key as keyof typeof a])) { return false; }
    }

    return true;
  }

  return a !== a && b !== b;
}
