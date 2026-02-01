interface P {
  work: () => void;
}

class SinhVien implements P {
  work {

  };
  age: number;
  name: string;

  sing() {
    console.log("I'm singing!");
  }
}

type Person = {
  age: number;
  name: string;
} & (Student | Pupil);

type Student = {
  type: "student";
  goToUniversity: () => void;
};

type Pupil = {
  type: "pupil";
  goToSchool: () => void;
};

let a: Person = {
  type: "pupil",
  name: "A",
  age: 15,
};

if (a.type === "student") {
  a.
}
