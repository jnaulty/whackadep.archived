<template>
  <div>
    <table
      class="table table-light table-striped table-hover table-bordered table-sm align-middle"
    >
      <thead style="position: sticky; top: 0">
        <tr>
          <th class="header" scope="col">name</th>
          <th class="header" scope="col">type</th>
          <th class="header" scope="col">rustsec</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="d in dependencies" v-bind:key="d.name">
          <td>{{ d.name }}</td>
          <td>
            <span v-if="d.direct">direct</span><span v-else>transitive</span>
          </td>
          <td>
            <span v-if="d.rustsec" :title="JSON.stringify(d.rustsec)">
              <strong>{{ d.rustsec.advisory.id }}</strong
              >: {{ d.rustsec.advisory.title }}.

              <span v-if="d.rustsec.version_info.patched.length > 0">
                <br />versions patched:
                {{ d.rustsec.version_info.patched.join(", ") }}.
              </span>

              <span v-if="d.rustsec.version_info.unaffected.length > 0">
                <br />versions unaffected:
                {{ d.rustsec.version_info.unaffected.join(", ") }}
              </span>
            </span>
          </td>
        </tr>
      </tbody>
    </table>
    <Modal ref="modal" />
  </div>
</template>

<script>
import Modal from "./Modal.vue";

export default {
  name: "DependenciesTable",
  props: {
    dependencies: Array,
  },
  components: {
    Modal,
  },
  inject: {
    semver: {
      from: "semver",
    },
  },
  methods: {
    clean_changelog(changelog) {
      var res = changelog.replaceAll(/(#)*/g, "");
      res = res.replaceAll(/\[([^\]]+)\]\([^)]+\)/g, "$1");
      return res.slice(0, 100);
    },
    version_change(dependency) {
      var version = dependency.version;
      var new_version =
        dependency.update.versions[dependency.update.versions.length - 1];
      // rust has the tendency to lie when

      var type_change = this.semver.diff(version, new_version);
      return type_change + " (" + version + " → " + new_version + ")";
    },
  },
};
</script>

<style scoped>
.header {
  position: sticky;
  top: 0;
}
</style>