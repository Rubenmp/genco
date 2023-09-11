package org.test;

import java.time.Instant;

import javax.persistence.Column;
import javax.persistence.Entity;
import javax.persistence.Id;
import javax.persistence.Table;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Entity
@Table(name = "table_name")
@NoArgsConstructor
@AllArgsConstructor
@Getter
@Setter
@Builder
public class DatabaseEntity {

  @Id
  @Column(name = "entity_id")
  private Long id;

  public String name = "adfa";

  protected Integer total;

  DatabaseEnum status;

  private Instant lastModifiedAt;

  private void method() {
  }

}
