extern crate textsearch;
extern crate futures;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use futures::Future;
use textsearch::global::Global;
use textsearch::text_index::Index;
use textsearch::search::Search;

static NAME: &'static str = "some index";

const DOCS: &'static [&'static str; 10] = &[

//EP1
"Turmoil has engulfed the
Galactic Republic. The taxation
of trade routes to outlying star
systems is in dispute.

Hoping to resolve the matter
with a blockade of deadly
battleships, the greedy Trade
Federation has stopped all
shipping to the small planet
of Naboo.

While the Congress of the
Republic endlessly debates
this alarming chain of events,
the Supreme Chancellor has
secretly dispatched two Jedi
Knights, the guardians of
peace and justice in the
galaxy, to settle the conflict",

//EP2
"There is unrest in the Galactic
Senate. Several thousand solar
systems have declared their
intentions to leave the Republic.

This separatist movement,
under the leadership of the
mysterious Count Dooku, has
made it difficult for the limited
number of Jedi Knights to maintain
peace and order in the galaxy.

Senator Amidala, the former
Queen of Naboo, is returning
to the Galactic Senate to vote
on the critical issue of creating
an ARMY OF THE REPUBLIC
to assist the overwhelmed
Jedi",

//EP3
"War! The Republic is crumbling
under attacks by the ruthless
Sith Lord, Count Dooku.
There are heroes on both sides.
Evil is everywhere.

In a stunning move, the
fiendish droid leader, General
Grievous, has swept into the
Republic capital and kidnapped
Chancellor Palpatine, leader of
the Galactic Senate.

As the Separatist Droid Army
attempts to flee the besieged
capital with their valuable
hostage, two Jedi Knights lead a
desperate mission to rescue the
captive Chancellor",


//EP4
"It is a period of civil war.
Rebel spaceships, striking
from a hidden base, have won
their first victory against
the evil Galactic Empire.

During the battle, Rebel
spies managed to steal secret
plans to the Empire's
ultimate weapon, the DEATH
STAR, an armored space
station with enough power
to destroy an entire planet.

Pursued by the Empire's
sinister agents, Princess
Leia races home aboard her
starship, custodian of the
stolen plans that can save her
people and restore
freedom to the galaxy",

//EP5
"It is a dark time for the
Rebellion. Although the Death
Star has been destroyed,
Imperial troops have driven the
Rebel forces from their hidden
base and pursued them across
the galaxy.

Evading the dreaded Imperial
Starfleet, a group of freedom
fighters led by Luke Skywalker
has established a new secret
base on the remote ice world
of Hoth.

The evil lord Darth Vader,
obsessed with finding young
Skywalker, has dispatched
thousands of remote probes into
the far reaches of space",

//EP6
"Luke Skywalker has returned to
his home planet of Tatooine in
an attempt to rescue his
friend Han Solo from the
clutches of the vile gangster
Jabba the Hutt.

Little does Luke know that the
GALACTIC EMPIRE has secretly
begun construction on a new
armored space station even
more powerful than the first
dreaded Death Star.

When completed, this ultimate
weapon will spell certain doom
for the small band of rebels
struggling to restore freedom
to the galaxy",

//EP7
"Luke Skywalker has vanished.
In his absence, the sinister
FIRST ORDER has risen from
the ashes of the Empire
and will not rest until
Skywalker, the last Jedi,
has been destroyed.

With the support of the
REPUBLIC, General Leia Organa
leads a brave RESISTANCE.
She is desperate to find her
brother Luke and gain his
help in restoring peace
and justice to the galaxy.

Leia has sent her most daring
pilot on a secret mission
to Jakku, where an old ally
has discovered a clue to
Luke's whereabouts",

//Vader quote
"I find your lack of faith disturbing",

//Obiwan quote
"The Force is what gives a Jedi his power. It's an energy field created by all living things. It surrounds us and penetrates us. It binds the galaxy together.",

//Something that has nothing to do with starwars
"Something completely different"
];

#[test]
fn create_search() {
	let search = Search::new();
}

#[test]
fn create_index_search() {
	let mut search = Search::new();

	let global: Arc<RwLock<Global>> = search.create_index(NAME).unwrap().clone();

	assert_eq!(global.read().unwrap().name, NAME);

	let indices = search.indices.clone();
	let indices = indices.read().unwrap();

	assert_eq!(indices.len(), 1);
}

#[test]
fn remove_index_search() {
	let mut search = Search::new();

	{
		search.create_index(NAME);
		let indices = search.indices.clone();
		let indices = indices.read().unwrap();
		assert_eq!(indices.len(), 1);
	}

	{
		search.remove_index(NAME);
		let indices = search.indices.clone();
		let indices = indices.read().unwrap();
		assert_eq!(indices.len(), 0);
	}
}

#[test]
fn insert_document_search() {
	let mut search = Search::new();

	search.create_index(NAME);

	let DOC_iter = DOCS.into_iter();
	let text_indices: Vec<Arc<Index>> = DOC_iter.map(|DOC| search.insert(NAME.to_string(), DOC.to_string()).wait().unwrap() ).collect();

	let indices = search.indices.clone();
	let indices = indices.read().unwrap();

	let global: Arc<RwLock<Global>> = indices.get(NAME).unwrap().clone();
	let global_indices: Vec<Arc<Index>> = global.read().unwrap().indices.clone();

	assert_eq!(text_indices.len(), DOCS.len());
	assert_eq!(text_indices.len(), global_indices.len());
}

#[test]
fn search_search() {
	let mut search = Search::new();

	search.create_index(NAME);
	let DOC_iter = DOCS.into_iter();
	let indices: Vec<Arc<Index>> = DOC_iter.map(|DOC| search.insert(NAME.to_string(), DOC.to_string()).wait().unwrap() ).collect();

	let scores: Vec<(Arc<Index>, f32)> = search.search(NAME.to_string(), "The force surrounds and penetrates us".to_string()).wait().unwrap();
	let first: Arc<Index> = scores[0].0.clone();

	assert_eq!(first.id, indices[8].id);
}
