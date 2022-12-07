const fs = require('fs');

type FileType = {
  name: string;
  size: number;
}

type DirType = {
  name: string;
  dirs: DirType[];
  files: FileType[];
  sizeMemo?: number;
}

function last<T>(arr: T[]): T {
  return arr[arr.length - 1];
}

function getDirSize(d: DirType): number {
  // if this has had its size measured already, return memo
  if (d.sizeMemo) return d.sizeMemo;

  let size = 0;
  // add direct files' sizes
  d.files.forEach(f => {
    size += f.size;
  });
  // then call this recursively on subfolders
  d.dirs.forEach(dir => {
    const subDirSize = dir.sizeMemo || getDirSize(dir);
    size += subDirSize;
  });
  // save this dir's size for future reference
  d.sizeMemo = size;
  return size;
}

function part1(root: DirType) {
  console.log("Part 1:");
  let totalsOver100k = 0;
  const dirsToTest: DirType[] = [root];
  while (dirsToTest.length > 0) {
    const dir = dirsToTest.pop();
    // console.log(`Testing ${dir?.name}`);
    if (dir) {
      const size = getDirSize(dir);
      // console.log(size);
      if (size <= 100000) {
        // console.log(`Dir ${dir.name} has size ${size}`);
        totalsOver100k += size;
      }
      // add this dir's dirs to test later
      dirsToTest.push(...dir.dirs);
    }
  }
  console.log("root size:", root.sizeMemo);
  console.log("total 100k sized sizes:", totalsOver100k);
}

function part2(root: DirType) {
  console.log("Part 2:");
  const fsSize = 70000000;
  const updateSize = 30000000;
  const rootSize = getDirSize(root);

  const neededSpareSize = updateSize - (fsSize - rootSize);
  console.log(`Required additional space: ${neededSpareSize}`);

  let smallestDirToDelete: DirType | undefined;
  const dirsToTest = [root];

  while (dirsToTest.length > 0) {
    const dir = dirsToTest.pop();
    if (dir && dir.sizeMemo) {
      if (dir.sizeMemo >= neededSpareSize) {
        // if the smallest dir is blank, or its size is larger than the dir we're testing
        // replace it
        if (!smallestDirToDelete || (smallestDirToDelete.sizeMemo && smallestDirToDelete.sizeMemo > dir.sizeMemo)) {
          smallestDirToDelete = dir;
        }
        // only push if this dir is big enough
        // if this dir is too small, so will all its children!
        dirsToTest.push(...dir.dirs);
      }
    }
  }

  console.log(`Please delete directory ${smallestDirToDelete?.name} with size ${smallestDirToDelete?.sizeMemo}`);
}

try {
  const data = fs.readFileSync('./src/07/input.txt', 'utf8');
  const lines = data.split('\n');

  const root: DirType = {
    name: "/",
    dirs: [],
    files: [],
  };

  const locationHistory: DirType[] = [root];

  lines.forEach((line: string) => {
    const lineParts = line.split(" ");
    const currentLocation = last(locationHistory);
    // console.log("location", locationHistory.map(dir => dir.name));

    if (lineParts[0] === "$") {
      // console.log(`Command: ${lineParts[1]}`);

      if (lineParts[1] === "cd") {
        const destination = lineParts[2];

        if (destination === "..") {
          locationHistory.pop();
        } else if (destination === "/") {
          // pop back to root
          while (locationHistory.length > 1) {
            locationHistory.pop();
          }
        } else {
          // going to a specific directory
          const possibleTarget = currentLocation.dirs.find(dir => dir.name === destination);
          if (possibleTarget) {
            locationHistory.push(possibleTarget);
          } else {
            const newDir: DirType = {
              name: destination,
              dirs: [],
              files: [],
            };
            // console.log("new directory", destination);
            currentLocation.dirs.push(newDir);
            locationHistory.push(last(currentLocation.dirs));
          }
        }
      } else if (lineParts[1] === "ls") {

      } else {
        throw(`Invalid command: ${lineParts[1]}`);
      }
    } else if (lineParts.length > 0) {
      if (lineParts[0] === "dir") {
        const [_, name] = lineParts;
        if (!currentLocation.dirs.find(dir => dir.name === name)) {
          // console.log("location", locationHistory.map(dir => dir.name));
          // console.log("new directory", name);
          currentLocation.dirs.push({
            name,
            files: [],
            dirs: [],
          });
        }

      } else {
        const [size, name] = lineParts;
        if (name && !currentLocation.files.find(f => f.name === name)) {
          // console.log("new file", name);
          currentLocation.files.push({
            name,
            size: parseInt(size, 10),
          });
        }
      }
    }
  });

  part1(root);
  console.log("");
  part2(root);

} catch (err) {
  console.error(err);
}
