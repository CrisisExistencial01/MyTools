#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>

#define OK(s) ("[\033[32m%s\033[0m] " s)
#define ERR(s) ("[\033[31m%s\033[0m] " s)
#define WARN(s) ("[\033[33m%s\033[0m] " s)

typedef struct {
    char *name;
    char *id;
} DockerImage;

void free_docker_images(DockerImage *images, int count) {
    for (int i = 0; i < count; i++) {
        free(images[i].name);
        free(images[i].id);
    }
    free(images);
}

DockerImage *find_all_docker_images(int *count) {
    FILE *fp;
    char buffer[512];
    DockerImage *images = NULL;
    int image_count = 0;

    fp = popen("docker images --format '{{.Repository}}:{{.Tag}} {{.ID}}'", "r");
    if (fp == NULL) {
        perror("popen");
        return NULL;
    }

    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
        buffer[strcspn(buffer, "\r\n")] = '\0';

        char *name = strtok(buffer, " ");
        char *id = strtok(NULL, " ");

        if (name && id) {
            images = realloc(images, sizeof(DockerImage) * (image_count + 1));
            if (!images) {
                perror("realloc");
                pclose(fp);
                return NULL;
            }

            images[image_count].name = strdup(name);
            images[image_count].id = strdup(id);
            image_count++;
        }
    }

    pclose(fp);
    *count = image_count;
    return images;
}

int is_malformed_image(const DockerImage *img) {
    if (strstr(img->name, "<none>") != NULL) {
        return 1;
    }
    return 0;
}

int remove_docker_image(const char *image_id) {
    char command[256];
    snprintf(command, sizeof(command), "docker rmi -f %s", image_id);

    int status = system(command);
    if (status == -1) {
        perror("system");
        return -1;
    } else if (WIFEXITED(status) && WEXITSTATUS(status) != 0) {
        fprintf(stderr, "%s Failed to remove image: %s\n", ERR("*"), image_id);
        return -1;
    }

    printf("%s Image %s removed successfully.\n", OK("*"), image_id);
    return 0;
}

int main(void) {
    int count = 0;
    DockerImage *images = find_all_docker_images(&count);
    if (!images) {
        fprintf(stderr, "No images found or failed to list images\n");
        return 1;
    }

    printf("Found %d images.\n", count);
    for (int i = 0; i < count; i++) {
        printf("Image: %s, ID: %s", images[i].name, images[i].id);
        if (is_malformed_image(&images[i])) {
            printf("\t-> %s Image marked as malformed. Removing...\n", WARN("*"));
            remove_docker_image(images[i].id);
        }
        printf("\n\n");
    }

    free_docker_images(images, count);
    return 0;
}
