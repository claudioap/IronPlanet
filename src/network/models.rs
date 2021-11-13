use crate::keys::*;
use crate::ShiftKey;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Serialize)]
pub(crate) struct BasicAuthCredentials<'a> {
    username: &'a str,
    password: &'a str,
    machine_uuid: Option<String>,
}

impl<'a> BasicAuthCredentials<'_> {
    pub(crate) fn new(
        username: &'a str,
        password: &'a str,
        machine_uuid: Option<String>,
    ) -> BasicAuthCredentials<'a> {
        BasicAuthCredentials {
            username,
            password,
            machine_uuid,
        }
    }
}

pub type AuthToken = String;

#[derive(Serialize)]
pub(crate) struct TokenCredentials {
    token: AuthToken,
}

impl TokenCredentials {
    pub(crate) fn new(token: AuthToken) -> TokenCredentials {
        TokenCredentials { token }
    }
}

#[derive(Deserialize)]
pub(crate) struct TokenResult {
    pub(crate) token: AuthToken,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum Weekday {
    Monday = 0,
    Thursday = 1,
    Wednesday = 2,
    Tuesday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum Period {
    Year = 1,
    FirstSemester = 2,
    SecondSemester = 3,
    FirstTrimester = 4,
    SecondTrimester = 5,
    ThirdTrimester = 6,
    FourthTrimester = 7,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum Degree {
    BSc = 1,
    MSc = 2,
    PhD = 3,
    IntegratedMSc = 4,
    PostGraduation = 5,
    AdvancedStudies = 6,
    PreGraduation = 7,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Language {
    #[serde(alias = "pt")]
    PT,
    #[serde(alias = "en")]
    EN,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum ShiftType {
    Theoretical = 1,
    Practical = 2,
    PracticalTheoretical = 3,
    Seminar = 4,
    TutorialOrientation = 5,
    FieldWork = 6,
    OnlineTheoretical = 7,
    OnlinePractical = 8,
    OnlinePracticalTheoretical = 9,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum FileCategory {
    Image = 1,
    Slides = 2,
    Problems = 3,
    Protocol = 4,
    Seminar = 5,
    Exam = 6,
    Test = 7,
    Support = 8,
    Others = 9,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum FileLicense {
    RightsReserved = 0,
    PublicDomain = 1,
    GPL = 2,
    MIT = 3,
    BSD = 4,
    CCBy = 5,
    CCBySa = 6,
    CCByNc = 7,
    CCBySaNc = 8,
    GenericPermissive = 100,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum FileVisibility {
    Public = 0,
    Students = 1,
    Enrolled = 2,
    Nobody = 3,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum Season {
    Normal = 1,
    Exam = 2,
    Special = 3,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum EventType {
    Test = 1,
    Exam = 2,
    Discussion = 3,
    FieldTrip = 4,
    ProjectAnnouncement = 5,
    ProjectDelivery = 6,
    AdditionalClass = 7,
    Presentation = 8,
    Seminar = 9,
    Talk = 10,
}

#[derive(Deserialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub(crate) enum RoomType {
    Generic = 1,
    Classroom = 2,
    Auditorium = 3,
    Laboratory = 4,
    Computer = 5,
    Meeting = 6,
    Masters = 7,
    Cabinet = 8,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Department {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) courses: Vec<CourseKey>,
    pub(crate) building: Option<BuildingKey>,
}

// #[derive(Deserialize, Debug, PartialEq)]
// #[serde(rename_all = "snake_case")]
// pub(crate) struct DepartmentPartial {
//     pub(crate) id: u32,
//     pub(crate) name: String,
// }

// #[derive(Deserialize, Debug, PartialEq)]
// #[serde(rename_all = "snake_case")]
// pub(crate) struct BuildingPartial {
//     pub(crate) id: u32,
//     pub(crate) name: String,
//     pub(crate) abbreviation: String,
// }

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Building {
    pub(crate) id: BuildingKey,
    pub(crate) name: String,
    pub(crate) abbreviation: String,
    pub(crate) places: Vec<RoomKey>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Place {
    pub(crate) id: PlaceKey,
    pub(crate) name: String,
    pub(crate) floor: i8,
    pub(crate) building: Option<BuildingKey>,
    pub(crate) picture: Option<String>,
    pub(crate) picture_cover: Option<String>,
    pub(crate) features: Vec<PlaceFeature>,
    pub(crate) room_meta: Option<Room>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct PlaceFeature {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) icon: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Room {
    // pub(crate) name: String,
    pub(crate) department: Option<DepartmentKey>,
    pub(crate) capacity: Option<u16>,
    pub(crate) door_number: Option<u16>,
    #[serde(rename = "type")]
    pub(crate) room_type: RoomType,
    pub(crate) description: Option<String>,
    pub(crate) equipment: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Course {
    pub(crate) id: CourseKey,
    pub(crate) abbreviation: String,
    pub(crate) name: String,
    pub(crate) degree: Degree,
    pub(crate) department: Option<DepartmentKey>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Class {
    pub(crate) id: ClassKey,
    pub(crate) name: String,
    pub(crate) abbreviation: String,
    pub(crate) credits: u32,
    pub(crate) department: Option<DepartmentKey>,
    pub(crate) instances: Vec<ClassInstanceKey>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ClassInstance {
    pub(crate) id: ClassInstanceKey,
    pub(crate) year: u32,
    pub(crate) department: Option<DepartmentKey>,
    pub(crate) period: Period,
    pub(crate) students: Vec<StudentKey>,
    pub(crate) teachers: Vec<TeacherKey>,
    pub(crate) shifts: Vec<ShiftKey>,
    // pub(crate) information: HashMap<String, ClassInfo>,
    pub(crate) information: ClassInfoSources,
    pub(crate) avg_grade: Option<f32>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ClassInfoSources {
    pub upstream: ClassInfo,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ClassInfo {
    pub program: ClassInfoEntry,
    pub assistance: ClassInfoEntry,
    pub extra_info: ClassInfoEntry,
    pub objectives: ClassInfoEntry,
    pub competences: ClassInfoEntry,
    pub description: ClassInfoEntry,
    pub bibliography: ClassInfoEntry,
    pub requirements: ClassInfoEntry,
    pub teaching_methods: ClassInfoEntry,
    pub evaluation_methods: ClassInfoEntry,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ClassInfoEntry {
    pub pt: Option<String>,
    pub en: Option<String>,
    pub time: Option<String>,
    pub editor: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ClassShift {
    pub(crate) id: ShiftKey,
    pub(crate) number: u16,
    #[serde(rename = "type")]
    pub(crate) shift_type: ShiftType,
    pub(crate) teachers: Vec<TeacherKey>,
    pub(crate) instances: Vec<ClassShiftInstance>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ClassShiftInstance {
    pub(crate) weekday: Weekday,
    pub(crate) start: u16,
    pub(crate) duration: u16,
    pub(crate) room: Option<RoomKey>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ClassInstanceFiles {
    pub(crate) official: Vec<ClassInstanceFile>,
    pub(crate) community: Vec<ClassInstanceFile>,
    // pub(crate) denied: Vec<ClassInstanceFile>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ClassInstanceFile {
    pub(crate) id: u32,
    pub(crate) file: File,
    pub(crate) name: String,
    pub(crate) category: FileCategory,
    pub(crate) upload_datetime: String,
    pub(crate) uploader: Option<u32>,
    pub(crate) uploader_teacher: Option<TeacherKey>,
    pub(crate) url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct File {
    pub(crate) hash: String,
    pub(crate) size: u32,
    pub(crate) mime: String,
    pub(crate) license: String,
    pub(crate) url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Student {
    pub(crate) id: StudentKey,
    pub(crate) name: String,
    pub(crate) abbreviation: Option<String>,
    pub(crate) number: u32,
    pub(crate) enrollments: Vec<EnrollmentKey>,
    pub(crate) shifts: Vec<ShiftKey>,
    pub(crate) first_year: Option<u32>,
    pub(crate) last_year: Option<u32>,
    pub(crate) course: Option<CourseKey>,
    pub(crate) avg_grade: Option<u32>,
    pub(crate) url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Teacher {
    pub(crate) id: TeacherKey,
    pub(crate) name: String,
    pub(crate) short_name: String,
    pub(crate) abbreviation: Option<String>,
    pub(crate) first_year: Option<u32>,
    pub(crate) last_year: Option<u32>,
    pub(crate) phone: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) thumb: Option<String>,
    pub(crate) rank: Option<String>,
    pub(crate) departments: Vec<DepartmentKey>,
    pub(crate) shifts: Vec<DepartmentKey>,
    pub(crate) url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Enrollment {
    pub(crate) id: EnrollmentKey,
    pub(crate) class_instance: ClassInstanceKey,
    pub(crate) student: StudentKey,
    pub(crate) attendance: bool,
    pub(crate) attendance_date: Option<String>,
    pub(crate) normal_grade: Option<u8>,
    pub(crate) normal_grade_date: Option<String>,
    pub(crate) recourse_grade: Option<u8>,
    pub(crate) recourse_grade_date: Option<String>,
    pub(crate) special_grade: Option<u8>,
    pub(crate) special_grade_date: Option<String>,
    pub(crate) improvement_grade: Option<u8>,
    pub(crate) improvement_grade_date: Option<String>,
    pub(crate) approved: bool,
    pub(crate) grade: Option<u8>,
}
