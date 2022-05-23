use std::{
	ops::{Deref, DerefMut},
};
use serde::{Deserialize, Serialize};

/// This struct allows for unique ids while still maintaining ordering
/// - Add to start and end
/// - Unique ID per element
/// - Maintain ordering
/// - Insert by unique id
/// - Remove elements without changing ordering or damaging unique ids
/// This functions somewhat similar to a linklist in terms of needs.
/// The downside is that currently it requires a lot of iteration.

type ElementId = u64;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VecUnique<T> {
	/// Map from element ids to array positions
	elements: Vec<T>,
	/// The IDs of the [Elements] contained within this
	element_ids: Vec<ElementId>,
	/// The ID that will be assigned to the next element that is added to this
	#[serde(skip)]
	next_id: ElementId,
}

impl<T> VecUnique<T> {
	/// Push a new element to the start of the vector
	pub fn push_front(&mut self, element: T) -> Option<ElementId> {
		self.next_id += 1;
		self.elements.insert(0, element);
		self.element_ids.insert(0, self.next_id);
		Some(self.next_id)
	}

	// Push an element to the end of the vector
	pub fn push_end(&mut self, element: T) -> Option<ElementId> {
		self.next_id += 1;
		self.elements.push(element);
		self.element_ids.push(self.next_id);
		Some(self.next_id)
	}

	/// Insert an element adjacent to the given ID
	pub fn insert(&mut self, element: T, id: ElementId) -> Option<ElementId> {
		if let Some(index) = self.index_from_id(id){
			self.next_id += 1;
			self.elements.insert(index, element,);
			self.element_ids.insert(index, self.next_id);
			return Some(self.next_id);
		}
		None
	}

	/// Add a range of elements of elements to the end of this vector
	pub fn push_range<I>(&mut self, elements: I) -> Vec<ElementId>
	where
		I: IntoIterator<Item = T>,
	{
		let mut ids = vec![];
		for element in elements {
			if let Some(id) = self.push_end(element) {
				ids.push(id);
			}
		}
		ids
	}

	/// Remove an element with a given element ID from the within this container.
	/// This operation will return false if the element ID is not found.
	/// Preserve unique ID lookup by using swap end and updating hashmap
	pub fn remove(&mut self, to_remove_id: ElementId) -> Option<T> {
		if let Some(index) = self.index_from_id(to_remove_id) {
			self.element_ids.remove(index);
			return Some(self.elements.remove(index));
		}
		None
	}

	/// Get a single element with a given element ID from the within this container.
	pub fn by_id(&self, id: ElementId) -> Option<&T> {
		let index = self.index_from_id(id)?;
		Some(&self.elements[index])
	}

	/// Get a mutable reference to a single element with a given element ID from the within this container.
	pub fn by_id_mut(&mut self, id: ElementId) -> Option<&mut T> {
		let index = self.index_from_id(id)?;
		Some(&mut self.elements[index])
	}

	/// Get an element based on its index
	pub fn by_index(&self, index: usize) -> Option<&T> {
		self.elements.get(index)
	}

	/// Get a mutable element based on its index
	pub fn by_index_mut(&mut self, index: usize) -> Option<&mut T> {
		self.elements.get_mut(index)
	}

	/// Clear the elements and unique ids
	pub fn clear(&mut self) {
		self.elements.clear();
		self.element_ids.clear();
	}

	/// Enumerate the ids and elements in this container `(&ElementId, &T)`
	pub fn enumerate(&self) -> impl Iterator<Item = (&ElementId, &T)> {
		self.element_ids.iter().zip(self.elements.iter())
	}

	/// Mutably Enumerate the ids and elements in this container `(&ElementId, &mut T)`
	pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (&ElementId, &mut T)> {
		self.element_ids.iter().zip(self.elements.iter_mut())
	}

	/// If this container contains an element with the given ID
	pub fn contains(&self, id: ElementId) -> bool {
		self.element_ids.contains(&id)
	}

	/// Get the index of an element with the given ID
	pub fn index_from_id(&self, element_id: ElementId) -> Option<usize> {
		// Though this is a linear traversal, it is still likely faster than using a hashmap
		self.element_ids.iter().position(|&id| id == element_id)
	}
}

impl<T> Default for VecUnique<T> {
	fn default() -> Self {
		VecUnique {
			elements: vec![],
			element_ids: vec![],
			next_id: 0,
		}
	}
}

/// Allows for usage of UniqueElements as a Vec<T>
impl<T> Deref for VecUnique<T> {
	type Target = [T];
	fn deref(&self) -> &Self::Target {
		&self.elements
	}
}

// TODO Consider removing this, it could allow for ElementIds and Elements to get out of sync
/// Allows for mutable usage of UniqueElements as a Vec<T>
impl<T> DerefMut for VecUnique<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.elements
	}
}

/// Allows use with iterators
/// Also allows constructing UniqueElements with collect
impl<A> FromIterator<A> for VecUnique<A> {
	fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
		let mut new = VecUnique::default();
		// Add to the end of the existing elements
		new.push_range(iter);
		new
	}
}