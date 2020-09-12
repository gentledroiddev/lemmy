use super::post_view::post_fast_view::BoxedQuery;
use crate::{fuzzy_search, limit_and_offset, ListingType, MaybeOptional, SortType};
use diesel::{dsl::*, pg::Pg, result::Error, *};
use serde::Serialize;

// The faked schema since diesel doesn't do views
table! {
  post_view (id) {
    id -> Int4,
    name -> Varchar,
    url -> Nullable<Text>,
    body -> Nullable<Text>,
    creator_id -> Int4,
    community_id -> Int4,
    removed -> Bool,
    locked -> Bool,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
    deleted -> Bool,
    nsfw -> Bool,
    stickied -> Bool,
    embed_title -> Nullable<Text>,
    embed_description -> Nullable<Text>,
    embed_html -> Nullable<Text>,
    thumbnail_url -> Nullable<Text>,
    ap_id -> Text,
    local -> Bool,
    creator_actor_id -> Text,
    creator_local -> Bool,
    creator_name -> Varchar,
    creator_preferred_username -> Nullable<Varchar>,
    creator_published -> Timestamp,
    creator_avatar -> Nullable<Text>,
    banned -> Bool,
    banned_from_community -> Bool,
    community_actor_id -> Text,
    community_local -> Bool,
    community_name -> Varchar,
    community_icon -> Nullable<Text>,
    community_removed -> Bool,
    community_deleted -> Bool,
    community_nsfw -> Bool,
    number_of_comments -> BigInt,
    score -> BigInt,
    upvotes -> BigInt,
    downvotes -> BigInt,
    hot_rank -> Int4,
    hot_rank_active -> Int4,
    newest_activity_time -> Timestamp,
    user_id -> Nullable<Int4>,
    my_vote -> Nullable<Int4>,
    subscribed -> Nullable<Bool>,
    read -> Nullable<Bool>,
    saved -> Nullable<Bool>,
  }
}

table! {
  post_fast_view (id) {
    id -> Int4,
    name -> Varchar,
    url -> Nullable<Text>,
    body -> Nullable<Text>,
    creator_id -> Int4,
    community_id -> Int4,
    removed -> Bool,
    locked -> Bool,
    published -> Timestamp,
    updated -> Nullable<Timestamp>,
    deleted -> Bool,
    nsfw -> Bool,
    stickied -> Bool,
    embed_title -> Nullable<Text>,
    embed_description -> Nullable<Text>,
    embed_html -> Nullable<Text>,
    thumbnail_url -> Nullable<Text>,
    ap_id -> Text,
    local -> Bool,
    creator_actor_id -> Text,
    creator_local -> Bool,
    creator_name -> Varchar,
    creator_preferred_username -> Nullable<Varchar>,
    creator_published -> Timestamp,
    creator_avatar -> Nullable<Text>,
    banned -> Bool,
    banned_from_community -> Bool,
    community_actor_id -> Text,
    community_local -> Bool,
    community_name -> Varchar,
    community_icon -> Nullable<Text>,
    community_removed -> Bool,
    community_deleted -> Bool,
    community_nsfw -> Bool,
    number_of_comments -> BigInt,
    score -> BigInt,
    upvotes -> BigInt,
    downvotes -> BigInt,
    hot_rank -> Int4,
    hot_rank_active -> Int4,
    newest_activity_time -> Timestamp,
    user_id -> Nullable<Int4>,
    my_vote -> Nullable<Int4>,
    subscribed -> Nullable<Bool>,
    read -> Nullable<Bool>,
    saved -> Nullable<Bool>,
  }
}

#[derive(Queryable, Identifiable, PartialEq, Debug, Serialize, QueryableByName, Clone)]
#[table_name = "post_fast_view"]
pub struct PostView {
  pub id: i32,
  pub name: String,
  pub url: Option<String>,
  pub body: Option<String>,
  pub creator_id: i32,
  pub community_id: i32,
  pub removed: bool,
  pub locked: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
  pub nsfw: bool,
  pub stickied: bool,
  pub embed_title: Option<String>,
  pub embed_description: Option<String>,
  pub embed_html: Option<String>,
  pub thumbnail_url: Option<String>,
  pub ap_id: String,
  pub local: bool,
  pub creator_actor_id: String,
  pub creator_local: bool,
  pub creator_name: String,
  pub creator_preferred_username: Option<String>,
  pub creator_published: chrono::NaiveDateTime,
  pub creator_avatar: Option<String>,
  pub banned: bool,
  pub banned_from_community: bool,
  pub community_actor_id: String,
  pub community_local: bool,
  pub community_name: String,
  pub community_icon: Option<String>,
  pub community_removed: bool,
  pub community_deleted: bool,
  pub community_nsfw: bool,
  pub number_of_comments: i64,
  pub score: i64,
  pub upvotes: i64,
  pub downvotes: i64,
  pub hot_rank: i32,
  pub hot_rank_active: i32,
  pub newest_activity_time: chrono::NaiveDateTime,
  pub user_id: Option<i32>,
  pub my_vote: Option<i32>,
  pub subscribed: Option<bool>,
  pub read: Option<bool>,
  pub saved: Option<bool>,
}

pub struct PostQueryBuilder<'a> {
  conn: &'a PgConnection,
  query: BoxedQuery<'a, Pg>,
  listing_type: ListingType,
  sort: &'a SortType,
  my_user_id: Option<i32>,
  for_creator_id: Option<i32>,
  for_community_id: Option<i32>,
  for_community_name: Option<String>,
  search_term: Option<String>,
  url_search: Option<String>,
  show_nsfw: bool,
  saved_only: bool,
  unread_only: bool,
  page: Option<i64>,
  limit: Option<i64>,
}

impl<'a> PostQueryBuilder<'a> {
  pub fn create(conn: &'a PgConnection) -> Self {
    use super::post_view::post_fast_view::dsl::*;

    let query = post_fast_view.into_boxed();

    PostQueryBuilder {
      conn,
      query,
      listing_type: ListingType::All,
      sort: &SortType::Hot,
      my_user_id: None,
      for_creator_id: None,
      for_community_id: None,
      for_community_name: None,
      search_term: None,
      url_search: None,
      show_nsfw: true,
      saved_only: false,
      unread_only: false,
      page: None,
      limit: None,
    }
  }

  pub fn listing_type(mut self, listing_type: ListingType) -> Self {
    self.listing_type = listing_type;
    self
  }

  pub fn sort(mut self, sort: &'a SortType) -> Self {
    self.sort = sort;
    self
  }

  pub fn for_community_id<T: MaybeOptional<i32>>(mut self, for_community_id: T) -> Self {
    self.for_community_id = for_community_id.get_optional();
    self
  }

  pub fn for_community_name<T: MaybeOptional<String>>(mut self, for_community_name: T) -> Self {
    self.for_community_name = for_community_name.get_optional();
    self
  }

  pub fn for_creator_id<T: MaybeOptional<i32>>(mut self, for_creator_id: T) -> Self {
    self.for_creator_id = for_creator_id.get_optional();
    self
  }

  pub fn search_term<T: MaybeOptional<String>>(mut self, search_term: T) -> Self {
    self.search_term = search_term.get_optional();
    self
  }

  pub fn url_search<T: MaybeOptional<String>>(mut self, url_search: T) -> Self {
    self.url_search = url_search.get_optional();
    self
  }

  pub fn my_user_id<T: MaybeOptional<i32>>(mut self, my_user_id: T) -> Self {
    self.my_user_id = my_user_id.get_optional();
    self
  }

  pub fn show_nsfw(mut self, show_nsfw: bool) -> Self {
    self.show_nsfw = show_nsfw;
    self
  }

  pub fn saved_only(mut self, saved_only: bool) -> Self {
    self.saved_only = saved_only;
    self
  }

  pub fn page<T: MaybeOptional<i64>>(mut self, page: T) -> Self {
    self.page = page.get_optional();
    self
  }

  pub fn limit<T: MaybeOptional<i64>>(mut self, limit: T) -> Self {
    self.limit = limit.get_optional();
    self
  }

  pub fn list(self) -> Result<Vec<PostView>, Error> {
    use super::post_view::post_fast_view::dsl::*;

    let mut query = self.query;

    query = match self.listing_type {
      ListingType::Subscribed => query.filter(subscribed.eq(true)),
      ListingType::Local => query.filter(community_local.eq(true)),
      _ => query,
    };

    if let Some(for_community_id) = self.for_community_id {
      query = query.filter(community_id.eq(for_community_id));
      query = query.then_order_by(stickied.desc());
    }

    if let Some(for_community_name) = self.for_community_name {
      query = query.filter(community_name.eq(for_community_name));
      query = query.then_order_by(stickied.desc());
    }

    if let Some(url_search) = self.url_search {
      query = query.filter(url.eq(url_search));
    }

    if let Some(search_term) = self.search_term {
      let searcher = fuzzy_search(&search_term);
      query = query
        .filter(name.ilike(searcher.to_owned()))
        .or_filter(body.ilike(searcher));
    }

    query = match self.sort {
      SortType::Active => query
        .then_order_by(hot_rank_active.desc())
        .then_order_by(published.desc()),
      SortType::Hot => query
        .then_order_by(hot_rank.desc())
        .then_order_by(published.desc()),
      SortType::New => query.then_order_by(published.desc()),
      SortType::TopAll => query.then_order_by(score.desc()),
      SortType::TopYear => query
        .filter(published.gt(now - 1.years()))
        .then_order_by(score.desc()),
      SortType::TopMonth => query
        .filter(published.gt(now - 1.months()))
        .then_order_by(score.desc()),
      SortType::TopWeek => query
        .filter(published.gt(now - 1.weeks()))
        .then_order_by(score.desc()),
      SortType::TopDay => query
        .filter(published.gt(now - 1.days()))
        .then_order_by(score.desc()),
    };

    // The view lets you pass a null user_id, if you're not logged in
    query = if let Some(my_user_id) = self.my_user_id {
      query.filter(user_id.eq(my_user_id))
    } else {
      query.filter(user_id.is_null())
    };

    // If its for a specific user, show the removed / deleted
    if let Some(for_creator_id) = self.for_creator_id {
      query = query.filter(creator_id.eq(for_creator_id));
    } else {
      query = query
        .filter(removed.eq(false))
        .filter(deleted.eq(false))
        .filter(community_removed.eq(false))
        .filter(community_deleted.eq(false));
    }

    if !self.show_nsfw {
      query = query
        .filter(nsfw.eq(false))
        .filter(community_nsfw.eq(false));
    };

    // TODO these are wrong, bc they'll only show saved for your logged in user, not theirs
    if self.saved_only {
      query = query.filter(saved.eq(true));
    };

    if self.unread_only {
      query = query.filter(read.eq(false));
    };

    let (limit, offset) = limit_and_offset(self.page, self.limit);
    query = query
      .limit(limit)
      .offset(offset)
      .filter(removed.eq(false))
      .filter(deleted.eq(false))
      .filter(community_removed.eq(false))
      .filter(community_deleted.eq(false));

    query.load::<PostView>(self.conn)
  }
}

impl PostView {
  pub fn read(
    conn: &PgConnection,
    from_post_id: i32,
    my_user_id: Option<i32>,
  ) -> Result<Self, Error> {
    use super::post_view::post_fast_view::dsl::*;
    use diesel::prelude::*;

    let mut query = post_fast_view.into_boxed();

    query = query.filter(id.eq(from_post_id));

    if let Some(my_user_id) = my_user_id {
      query = query.filter(user_id.eq(my_user_id));
    } else {
      query = query.filter(user_id.is_null());
    };

    query.first::<Self>(conn)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    community::*,
    post::*,
    post_view::*,
    tests::establish_unpooled_connection,
    user::*,
    Crud,
    Likeable,
    *,
  };

  #[test]
  fn test_crud() {
    let conn = establish_unpooled_connection();

    let user_name = "tegan".to_string();
    let community_name = "test_community_3".to_string();
    let post_name = "test post 3".to_string();

    let new_user = UserForm {
      name: user_name.to_owned(),
      preferred_username: None,
      password_encrypted: "nope".into(),
      email: None,
      matrix_user_id: None,
      avatar: None,
      banner: None,
      updated: None,
      admin: false,
      banned: false,
      show_nsfw: false,
      theme: "darkly".into(),
      default_sort_type: SortType::Hot as i16,
      default_listing_type: ListingType::Subscribed as i16,
      lang: "browser".into(),
      show_avatars: true,
      send_notifications_to_email: false,
      actor_id: None,
      bio: None,
      local: true,
      private_key: None,
      public_key: None,
      last_refreshed_at: None,
    };

    let inserted_user = User_::create(&conn, &new_user).unwrap();

    let new_community = CommunityForm {
      name: community_name.to_owned(),
      title: "nada".to_owned(),
      description: None,
      creator_id: inserted_user.id,
      category_id: 1,
      removed: None,
      deleted: None,
      updated: None,
      nsfw: false,
      actor_id: None,
      local: true,
      private_key: None,
      public_key: None,
      last_refreshed_at: None,
      published: None,
      icon: None,
      banner: None,
    };

    let inserted_community = Community::create(&conn, &new_community).unwrap();

    let new_post = PostForm {
      name: post_name.to_owned(),
      url: None,
      body: None,
      creator_id: inserted_user.id,
      community_id: inserted_community.id,
      removed: None,
      deleted: None,
      locked: None,
      stickied: None,
      updated: None,
      nsfw: false,
      embed_title: None,
      embed_description: None,
      embed_html: None,
      thumbnail_url: None,
      ap_id: None,
      local: true,
      published: None,
    };

    let inserted_post = Post::create(&conn, &new_post).unwrap();

    let post_like_form = PostLikeForm {
      post_id: inserted_post.id,
      user_id: inserted_user.id,
      score: 1,
    };

    let inserted_post_like = PostLike::like(&conn, &post_like_form).unwrap();

    let expected_post_like = PostLike {
      id: inserted_post_like.id,
      post_id: inserted_post.id,
      user_id: inserted_user.id,
      published: inserted_post_like.published,
      score: 1,
    };

    let read_post_listings_with_user = PostQueryBuilder::create(&conn)
      .listing_type(ListingType::Community)
      .sort(&SortType::New)
      .for_community_id(inserted_community.id)
      .my_user_id(inserted_user.id)
      .list()
      .unwrap();

    let read_post_listings_no_user = PostQueryBuilder::create(&conn)
      .listing_type(ListingType::Community)
      .sort(&SortType::New)
      .for_community_id(inserted_community.id)
      .list()
      .unwrap();

    let read_post_listing_no_user = PostView::read(&conn, inserted_post.id, None).unwrap();
    let read_post_listing_with_user =
      PostView::read(&conn, inserted_post.id, Some(inserted_user.id)).unwrap();

    // the non user version
    let expected_post_listing_no_user = PostView {
      user_id: None,
      my_vote: None,
      id: inserted_post.id,
      name: post_name.to_owned(),
      url: None,
      body: None,
      creator_id: inserted_user.id,
      creator_name: user_name.to_owned(),
      creator_preferred_username: None,
      creator_published: inserted_user.published,
      creator_avatar: None,
      banned: false,
      banned_from_community: false,
      community_id: inserted_community.id,
      removed: false,
      deleted: false,
      locked: false,
      stickied: false,
      community_name: community_name.to_owned(),
      community_icon: None,
      community_removed: false,
      community_deleted: false,
      community_nsfw: false,
      number_of_comments: 0,
      score: 1,
      upvotes: 1,
      downvotes: 0,
      hot_rank: read_post_listing_no_user.hot_rank,
      hot_rank_active: read_post_listing_no_user.hot_rank_active,
      published: inserted_post.published,
      newest_activity_time: inserted_post.published,
      updated: None,
      subscribed: None,
      read: None,
      saved: None,
      nsfw: false,
      embed_title: None,
      embed_description: None,
      embed_html: None,
      thumbnail_url: None,
      ap_id: inserted_post.ap_id.to_owned(),
      local: true,
      creator_actor_id: inserted_user.actor_id.to_owned(),
      creator_local: true,
      community_actor_id: inserted_community.actor_id.to_owned(),
      community_local: true,
    };

    let expected_post_listing_with_user = PostView {
      user_id: Some(inserted_user.id),
      my_vote: Some(1),
      id: inserted_post.id,
      name: post_name,
      url: None,
      body: None,
      removed: false,
      deleted: false,
      locked: false,
      stickied: false,
      creator_id: inserted_user.id,
      creator_name: user_name,
      creator_preferred_username: None,
      creator_published: inserted_user.published,
      creator_avatar: None,
      banned: false,
      banned_from_community: false,
      community_id: inserted_community.id,
      community_name,
      community_icon: None,
      community_removed: false,
      community_deleted: false,
      community_nsfw: false,
      number_of_comments: 0,
      score: 1,
      upvotes: 1,
      downvotes: 0,
      hot_rank: read_post_listing_with_user.hot_rank,
      hot_rank_active: read_post_listing_with_user.hot_rank_active,
      published: inserted_post.published,
      newest_activity_time: inserted_post.published,
      updated: None,
      subscribed: Some(false),
      read: Some(false),
      saved: Some(false),
      nsfw: false,
      embed_title: None,
      embed_description: None,
      embed_html: None,
      thumbnail_url: None,
      ap_id: inserted_post.ap_id.to_owned(),
      local: true,
      creator_actor_id: inserted_user.actor_id.to_owned(),
      creator_local: true,
      community_actor_id: inserted_community.actor_id.to_owned(),
      community_local: true,
    };

    let like_removed = PostLike::remove(&conn, inserted_user.id, inserted_post.id).unwrap();
    let num_deleted = Post::delete(&conn, inserted_post.id).unwrap();
    Community::delete(&conn, inserted_community.id).unwrap();
    User_::delete(&conn, inserted_user.id).unwrap();

    // The with user
    assert_eq!(
      expected_post_listing_with_user,
      read_post_listings_with_user[0]
    );
    assert_eq!(expected_post_listing_with_user, read_post_listing_with_user);
    assert_eq!(1, read_post_listings_with_user.len());

    // Without the user
    assert_eq!(expected_post_listing_no_user, read_post_listings_no_user[0]);
    assert_eq!(expected_post_listing_no_user, read_post_listing_no_user);
    assert_eq!(1, read_post_listings_no_user.len());

    // assert_eq!(expected_post, inserted_post);
    // assert_eq!(expected_post, updated_post);
    assert_eq!(expected_post_like, inserted_post_like);
    assert_eq!(1, like_removed);
    assert_eq!(1, num_deleted);
  }
}
