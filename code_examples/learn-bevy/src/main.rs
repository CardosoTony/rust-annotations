use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run()
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                print_names,
                people_with_jobs,
                people_ready_for_hire,
                person_does_job,
            ),
        );
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Kyle".to_string(),
        },
        Employed {
            job: Job::Developer,
        },
    ));

    // This entity doesn't have the 'Employed' Component.
    commands.spawn(Person {
        name: "Jane".to_string(),
    });

    commands.spawn((
        Person {
            name: "John".to_string(),
        },
        Employed {
            job: Job::Programmer,
        },
    ));

    commands.spawn((
        Person {
            name: "Peter".to_string(),
        },
        Employed { job: Job::Engineer },
    ));

    // This entity doesn't have the 'Employed' Component.
    commands.spawn((Person {
        name: "Dorothy".to_string(),
    },));

    commands.spawn((
        Person {
            name: "Mary".to_string(),
        },
        Employed {
            job: Job::Developer,
        },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire.", person.name);
    }
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Developer => "Developer",
            Job::Programmer => "Programmer",
            Job::Engineer => "Engineer",
        };
        println!("{0} is a {1}.", person.name, job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Developer,
    Programmer,
    Engineer,
}
