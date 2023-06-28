from dataclasses import dataclass

@dataclass(slots=True, order=True)
class Version:
    major: int
    minor: int
    
    @classmethod
    def from_str(cls, s: str):
        major = int(s[:2])
        minor = int(s[2:])
        return Version(major, minor)

    def __repr__(self) -> str:
        return str(self.major) + '.' + str(self.minor)